use super::{
    commands::*, events::*, AttributeType, AttributesBox, AttributesType, CommandType, EffectType,
    Entity, EventType, Position, TaskType, World, WorldType,
};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc::Sender;

pub struct GameAttributes {
    pub attributes: AttributesType,
}

impl GameAttributes {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
}

impl AttributesBox for GameAttributes {
    fn attributes(&self) -> &AttributesType {
        &self.attributes
    }
}

pub type GameAttributesType = Arc<Mutex<GameAttributes>>;

pub struct Game {
    pub attributes: GameAttributesType,
    pub listeners: Rc<RwLock<HashMap<String, Vec<EffectType>>>>,
    pub tasker: Sender<TaskType>,
    pub world: WorldType,
}

impl Game {
    pub fn new(tasker: Sender<TaskType>) -> Self {
        Self {
            attributes: Arc::new(Mutex::new(GameAttributes::new())),
            listeners: Rc::new(RwLock::new(HashMap::new())),
            tasker,
            world: Arc::new(Mutex::new(World::new())),
        }
    }

    pub fn process(&mut self, command_box: CommandType) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        // TODO: maybe instead implement separate process fn for command types and call them here
        // println!("Process {:?}", command_box);
        if let Some(EmitEventCommand(event)) =
            command_box.as_any().downcast_ref::<EmitEventCommand>()
        {
            let event = event.clone();
            return self.emit_event(event);
        } else if let Some(_) = command_box.as_any().downcast_ref::<SetWorldCommand>() {
            let SetWorldCommand(world) = **command_box
                .as_any_box()
                .downcast::<Box<SetWorldCommand>>()
                .unwrap();
            self.set_world(world);
        } else if let Some(_) = command_box
            .as_any()
            .downcast_ref::<SetGameAttributeCommand>()
        {
            let SetGameAttributeCommand(attribute) = **command_box
                .as_any_box()
                .downcast::<Box<SetGameAttributeCommand>>()
                .unwrap();
            self.set_attribute(attribute);
        } else if let Some(_) = command_box.as_any().downcast_ref::<AddEntityCommand>() {
            let AddEntityCommand { position, entity } = **command_box
                .as_any_box()
                .downcast::<Box<AddEntityCommand>>()
                .unwrap();
            self.add_entity(position, entity);
        } else if let Some(_) = command_box.as_any().downcast_ref::<MoveEntityCommand>() {
            let MoveEntityCommand { from, to } = **command_box
                .as_any_box()
                .downcast::<Box<MoveEntityCommand>>()
                .unwrap();
            return self.move_entity(from, to);
        } else if let Some(_) = command_box
            .as_any()
            .downcast_ref::<SetEntityAttributeCommand>()
        {
            let SetEntityAttributeCommand {
                position,
                attribute,
            } = **command_box
                .as_any_box()
                .downcast::<Box<SetEntityAttributeCommand>>()
                .unwrap();
            return self.set_entity_attribute(position, attribute);
        }
        Box::pin(async {})
    }

    fn set_attribute(&mut self, attribute: AttributeType) {
        let mut attributes = self.attributes.lock().unwrap();
        let key = attribute.as_name().to_string();
        attributes.attributes.insert(key, attribute);
        // TODO: emit event
    }

    fn set_world(&mut self, world: World) {
        self.world = Arc::new(Mutex::new(world));
        // TODO: emit event
    }

    fn add_entity(&mut self, position: Position, entity: Entity) {
        if let Some(tile) = self.world.lock().unwrap().0.get_mut(&position) {
            tile.entities.push(entity);
            // TODO: emit event
        }
    }

    fn move_entity(
        &mut self,
        from: Position,
        to: Position,
    ) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async move {
            if let Some(event) = (|| {
                let mut world = self.world.lock().unwrap();
                let from_tile = world.tile_mut(&from)?;
                let entity = from_tile.entities.remove(from.stack_pos? as usize);
                let to_tile = world.tile_mut(&to)?;
                to_tile.entities.push(entity);
                Some(Arc::new(MovedEntityEvent { from, to }) as EventType)
            })() {
                self.emit_event(event).await;
            }
        })
    }

    fn set_entity_attribute(
        &mut self,
        position: Position,
        attribute: AttributeType,
    ) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async move {
            if let Some(event) = (|| {
                let mut world = self.world.lock().unwrap();
                let entity = world.entity_mut(&position)?;
                let attribute_name = attribute.as_name().to_string();
                entity.attributes.insert(attribute_name.clone(), attribute);
                Some(Arc::new(ChangedEntityEvent {
                    position,
                    attribute_name,
                }) as EventType)
            })() {
                self.emit_event(event).await;
            }
        })
    }

    // fn remove_entity_attribute(&mut self, position: Position, attribute: AttributeType) {
    //     // TODO: emit event
    // }

    fn emit_event(&mut self, event: EventType) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        // println!("Event {:?}", event);
        Box::pin(async move {
            let event_name = event.as_name();
            if let Some(listeners) = self.listeners.clone().read().unwrap().get(event_name) {
                for listener in listeners.iter() {
                    if let Some((commands, tasks)) =
                        listener(event.clone(), self.attributes.clone(), self.world.clone())
                    {
                        for command in commands.into_iter() {
                            self.process(command).await;
                        }
                        for task in tasks.into_iter() {
                            self.tasker.send(task).await.unwrap(); // FIXME: unwrap
                        }
                    }
                }
            }
        })
    }
}
