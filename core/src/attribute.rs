use std::{any::Any, collections::HashMap};

pub trait Attribute: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_name(&self) -> &str;
}

pub type AttributeType = Box<dyn Attribute>;
pub type AttributesType = HashMap<String, AttributeType>;
