use super::super::{definitions::Packet, events::MovePayloadEvent, payload::Payload};
use crate::core::prelude::*;
use crate::systems::prelude::*;
use futures::FutureExt;
use std::time::{Duration, Instant};

system! {
    #[effect(MovePayloadEvent)]
    fn handle_move_payload(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        let MovePayloadEvent { player, direction } = event.as_any().downcast_ref::<MovePayloadEvent>().unwrap();
        let world = world.lock().unwrap();
        let mut commands = Vec::new();
        let mut tasks = Vec::new();

        if let Some((command, task)) = (|| {
            let position_and_player_entity = world.0.iter().enumerate()
                .find_map(|(index, (pos, tile))| tile.entities.iter().find(|entity| match entity.player() {
                    Some(Player(player_id)) => *player_id == player.0,
                    _ => false,
                })
                .map(|entity| (pos.clone().stack_pos(Some(index as u16)), entity)))?;
            let (position, player_entity) = position_and_player_entity;
            let can_walk = player_entity.walking().map_or_else(|| true, |w| Instant::now().checked_duration_since(w.until).is_some());
            if can_walk {
                let player_speed = player_entity.speed()?;
                let ground_speed = 150;
                let command = Box::new(SetEntityAttributeCommand {
                    position: position.clone(),
                    attribute: Box::new(Walking {
                        until: Instant::now() + Duration::from_millis((1000 * ground_speed as u64) / player_speed.0 as u64),
                    }),
                }) as CommandType;
                let event = Arc::new(MoveEvent {
                    from: position.clone(),
                    to: direction.clone().apply_to_position(position.clone()),
                    player: Some(player.clone()),
                }) as EventType;
                Some((
                    command,
                    Box::pin(async move {
                        Some(event)
                    }.into_stream()) as TaskType,
                ))
            } else {
                None
            }
        })() {
            commands.push(command);
            tasks.push(task);
        }

        Some((commands, tasks))
    }

    #[effect(MoveEvent)]
    fn handle_move(event: EventType, _: GameAttributesType, world: WorldType) -> EffectResultType {
        let MoveEvent { from, to, player } = event.as_any().downcast_ref::<MoveEvent>().unwrap();
        let world = world.lock().unwrap();
        let mut commands = Vec::new();

        if let Some(command) = (|| {
            let from_tile = world.tile(from)?;
            let id = player.clone()?.0;
            let player_entity_pos = from_tile.entities.iter().position(|entity| match entity.player() {
                Some(Player(player_id)) => *player_id == id,
                _ => false,
            })?;
            Some(Box::new(MoveEntityCommand {
                from: from.clone().stack_pos(Some(player_entity_pos as u16)),
                to: to.clone(),
            }) as CommandType)
        })() {
            commands.push(command);
        }

        Some((commands, Vec::new()))
    }

    #[effect(MovedEntityEvent)]
    fn handle_moved_entity(event: EventType, attributes: GameAttributesType, world: WorldType) -> EffectResultType {
        let MovedEntityEvent { from, to } = event.as_any().downcast_ref::<MovedEntityEvent>().unwrap();
        let world_clone = world.clone();
        let world_lock = world_clone.lock().unwrap();
        let mut commands = Vec::new();
        let mut tasks = Vec::new();

        let from_tile = world_lock.tile(&from)?;
        let to_tile = world_lock.tile(&to)?;
        let entity_stack_pos = (to_tile.entities.len() - 1) as u16;
        let entity = to_tile.entities.get(entity_stack_pos as usize).unwrap();

        if let Some(events) = (|| {
            let entity_position = to.clone().stack_pos(Some(entity_stack_pos));
            let mut events = Vec::new();
            for (stack_pos, _) in from_tile.entities.iter().enumerate() {
                events.push(Arc::new(SeparationEvent {
                    first: from.clone().stack_pos(Some(stack_pos as u16)),
                    second: entity_position.clone(),
                }) as EventType);
            }
            for (stack_pos, _) in to_tile.entities[0..to_tile.entities.len() - 1].iter().enumerate() {
                events.push(Arc::new(CollisionEvent {
                    first: to.clone().stack_pos(Some(stack_pos as u16)),
                    second: entity_position.clone(),
                }) as EventType);
            }
            Some(events)
        })() {
            let mut events = events.into_iter().map(|event| Box::new(EmitEventCommand(event)) as CommandType).collect();
            commands.append(&mut events);
        }

        if let Some(task) = (|| {
            let world = world.clone();
            let player = entity.player()?;
            let game_attributes = attributes.lock().unwrap();
            let clients = game_attributes.clients()?;
            let client = clients.0.get(player)?.clone();
            let direction = Direction::between_positions(from.clone(), to.clone())?;
            let map_payload = match direction.0 {
                Directions::North => Some(Payload::MapNorth { position: from.clone(), world }),
                Directions::West => Some(Payload::MapWest { position: to.clone(), world }),
                Directions::South => Some(Payload::MapSouth { position: to.clone(), world }),
                Directions::East => Some(Payload::MapEast { position: to.clone(), world }),
                _ => None,
            }?;
            let from_stack_pos = from.stack_pos.map(|pos| if pos == 0 { pos } else { from_tile.entities.len() as u16 - pos + 1 });
            let packet = Packet(Payload::Combined(vec![
                Payload::MovedEntity { from: from.clone().stack_pos(from_stack_pos), to: to.clone() },
                map_payload,
            ]));
            Some(Box::pin(async move {
                client.0.send(packet).await;
                None
            }.into_stream()) as TaskType)
        })() {
            tasks.push(task);
        }

        Some((commands, tasks))
    }
}
