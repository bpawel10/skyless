use super::{AttributesType, Entity};

#[derive(Debug)]
pub struct Tile {
    pub attributes: AttributesType,
    pub entities: Vec<Entity>,
}
