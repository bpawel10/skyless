use super::{AttributesBox, AttributesType, Entity};

pub struct Tile {
    pub attributes: AttributesType,
    pub entities: Vec<Entity>,
}

impl AttributesBox for Tile {
    fn attributes(&self) -> &AttributesType {
        &self.attributes
    }
}
