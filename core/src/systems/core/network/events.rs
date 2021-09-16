use crate::core::prelude::*;
use crate::systems::prelude::*;

#[event]
pub struct PingPayloadEvent;

#[event]
pub struct PingBackPayloadEvent;

#[event]
pub struct MovePayloadEvent {
    pub player: Player,
    pub direction: Direction,
}

#[event]
pub struct UseItemPayloadEvent {
    pub position: Position,
    pub item: Item,
}
