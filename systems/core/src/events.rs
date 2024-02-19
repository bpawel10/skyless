use crate::prelude::*;
use skyless_core::prelude::*;

#[event]
pub struct MoveEvent {
    pub from: Position,
    pub to: Position,
    pub player: Option<Player>, // TODO: use generic 'Id' attribute instead?
}

#[event]
pub struct CollisionEvent {
    pub first: Position,
    pub second: Position,
}

#[event]
pub struct SeparationEvent {
    pub first: Position,
    pub second: Position,
}

#[event]
pub struct UseEvent {
    pub source: Option<Position>,
    pub target: Position,
}
