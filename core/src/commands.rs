use super::{AttributeType, Command, Entity, EventType, Position, World};
use skyless_macro::command;
use std::any::Any;

#[command]
pub struct EmitEventCommand(pub EventType);

#[command]
pub struct SetGameAttributeCommand(pub AttributeType);

#[command]
pub struct SetWorldCommand(pub World);

#[command]
pub struct AddEntityCommand {
    pub position: Position,
    pub entity: Entity,
}

#[command]
pub struct SetEntityAttributeCommand {
    pub position: Position,
    pub attribute: AttributeType,
}

#[command]
pub struct RemoveEntityAttributeCommand {
    pub position: Position,
    pub attribute: AttributeType,
}

#[command]
pub struct MoveEntityCommand {
    pub from: Position,
    pub to: Position,
}
