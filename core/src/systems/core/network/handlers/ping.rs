use super::super::{
    definitions::{Client, Packet},
    payload::Payload,
};
use crate::core::prelude::*;
use crate::systems::prelude::*;
use futures::FutureExt;

system! {
     #[effect(PingPayloadEvent)]
    fn handle_ping_payload(_: EventType, attributes: GameAttributesType, _: WorldType) -> EffectResultType {
        let mut tasks = Vec::new();
        let game_attributes = attributes.lock().unwrap();
        if let Some(clients) = game_attributes.clients() {
            if let Some(Client(client)) = clients.0.get(&Player(1)) { // TODO: get rid of hardcoded player id
                let client = client.clone();
                tasks.push(Box::pin(async move {
                    client.send(Packet(Payload::Ping)).await;
                    None
                }.into_stream()) as TaskType);
            }
        }
        Some((Vec::new(), tasks))
    }

    #[effect(PingBackPayloadEvent)]
    fn handle_ping_back_payload(_: EventType, attributes: GameAttributesType, _: WorldType) -> EffectResultType {
        let mut tasks = Vec::new();
        let game_attributes = attributes.lock().unwrap();
        if let Some(clients) = game_attributes.clients() {
            if let Some(Client(client)) = clients.0.get(&Player(1)) { // TODO: get rid of hardcoded player id
                let client = client.clone();
                tasks.push(Box::pin(async move {
                    client.send(Packet(Payload::Ping)).await;
                    None
                }.into_stream()) as TaskType);
            }
        }
        Some((Vec::new(), tasks))
    }
}
