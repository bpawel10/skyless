use std::{any::Any, fmt::Debug};

pub trait Command: Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

pub type CommandType = Box<dyn Command>;
pub type CommandsType = Vec<CommandType>;
