use super::{CommandsType, EventType, GameAttributesType, TasksType, WorldType};

pub type EffectResultType = Option<(CommandsType, TasksType)>;
pub type EffectType =
    Box<dyn Fn(EventType, GameAttributesType, WorldType) -> EffectResultType + Send>;
