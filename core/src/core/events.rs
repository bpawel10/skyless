use super::{Event, Position};
use skyless_macros::event;
use std::any::Any;

#[event]
pub struct SystemsLoadedEvent;

#[event]
pub struct MovedEntityEvent {
    pub from: Position,
    pub to: Position,
}

#[event]
pub struct ChangedEntityEvent {
    pub position: Position,
    pub attribute_name: String,
}
