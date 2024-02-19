use skyless_core::{prelude::*, CommandType};
use skyless_systems_core::prelude::*;

system! {
    #[effect(CollisionEvent)]
    fn handle_switch_collision(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        println!("Handle switch collision");
        let CollisionEvent { first, second } = event.as_any().downcast_ref::<CollisionEvent>().unwrap();
        let mut commands = Vec::new();
        let world = world.lock().unwrap();
        if let Some(command) = {
            let first_entity = world.entity(first)?;
            match first_entity.action()? {
                Action(Actions::Switch) => {
                    let second_entity = world.entity(second)?;
                    let _ = second_entity.player()?;
                    println!("Activate switch");
                    Some(Box::new(SetEntityAttributeCommand {
                        position: first.clone(),
                        attribute: Box::new(Item(Items::StoneSwitchActivated.into())),
                    }) as CommandType)
                },
                _ => None
            }
        } {
            commands.push(command);
        }
        Some((commands, Vec::new()))
    }

    #[effect(SeparationEvent)]
    fn handle_switch_separation(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        println!("Handle switch separation");
        let SeparationEvent { first, second } = event.as_any().downcast_ref::<SeparationEvent>().unwrap();
        let mut commands = Vec::new();
        let world = world.lock().unwrap();
        if let Some(command) = {
            let first_entity = world.entity(first)?;
            match first_entity.action()? {
                Action(Actions::Switch) => {
                    let second_entity = world.entity(second)?;
                    let _ = second_entity.player()?;
                    println!("Deactivate switch");
                    Some(Box::new(SetEntityAttributeCommand {
                        position: first.clone(),
                        attribute: Box::new(Item(Items::StoneSwitch.into())),
                    }) as CommandType)
                },
                _ => None,
            }
        } {
            commands.push(command);
        }
        Some((commands, Vec::new()))
    }
}
