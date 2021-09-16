use super::EventType;
use std::pin::Pin;
use tokio_stream::Stream;

pub type TaskType = Pin<Box<dyn Stream<Item = Option<EventType>> + Send>>;
pub type TasksType = Vec<TaskType>;
