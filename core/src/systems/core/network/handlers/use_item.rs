use super::super::events::UseItemPayloadEvent;
use crate::core::prelude::*;
use crate::systems::prelude::*;

system! {
    #[effect(UseItemPayloadEvent)]
    fn handle_use_item_payload(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        let UseItemPayloadEvent { position, item } = event.as_any().downcast_ref::<UseItemPayloadEvent>().unwrap();
        let world = world.lock().unwrap();
        let mut commands = Vec::new();

        if let Some(event) = (|| {
            let entity = world.entity(position)?;
            let entity_item = entity.item()?;
            if entity_item.0 == item.0 {
                Some(Arc::new(UseEvent {
                    source: None,
                    target: position.clone(),
                }))
            } else {
                None
            }
        })() {
            commands.push(Box::new(EmitEventCommand(event)) as CommandType);
        }

        Some((commands, Vec::new()))
    }
}
