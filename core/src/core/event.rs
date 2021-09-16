use std::{any::Any, fmt::Debug, sync::Arc};

pub trait Event: Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_name(&self) -> &str;
}

pub type EventType = Arc<dyn Event + 'static>;
pub type EventsType = Vec<EventType>;
