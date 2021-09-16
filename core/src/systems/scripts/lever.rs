use crate::core::prelude::*;
use crate::systems::prelude::*;
use std::convert::TryInto;

system! {
    #[effect(UseEvent)]
    fn handle_lever_use(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        println!("Handle lever use");
        let UseEvent { target, .. } = event.as_any().downcast_ref::<UseEvent>().unwrap();
        let mut commands = Vec::new();
        let world = world.lock().unwrap();

        if let Some(command) = (|| {
            let entity = world.entity(target)?;
            match entity.action()? {
                Action(Actions::Lever) => {
                    let entity_item = entity.item()?;
                    let new_item = match entity_item.0.try_into().unwrap() {
                        Items::LeverLeft => Some(Items::LeverRight),
                        Items::LeverRight => Some(Items::LeverLeft),
                        _ => None,
                    }?;
                    Some(Box::new(SetEntityAttributeCommand {
                        position: target.clone(),
                        attribute: Box::new(Item(new_item.into())),
                    }) as CommandType)
                },
                _ => None
            }
        })() {
            commands.push(command);
        }

        Some((commands, Vec::new()))
    }
}
