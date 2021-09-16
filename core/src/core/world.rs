use super::{Entity, Position, Tile};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct World(pub HashMap<Position, Tile>);

pub type WorldType = Arc<Mutex<World>>;

impl World {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn tile(&self, position: &Position) -> Option<&Tile> {
        self.0.get(&position.clone().stack_pos(None))
    }

    pub fn tile_mut(&mut self, position: &Position) -> Option<&mut Tile> {
        self.0.get_mut(&position.clone().stack_pos(None))
    }

    pub fn entity(&self, position: &Position) -> Option<&Entity> {
        let tile = self.tile(position)?;
        tile.entities.get(position.stack_pos? as usize)
    }

    pub fn entity_mut(&mut self, position: &Position) -> Option<&mut Entity> {
        let tile = self.tile_mut(position)?;
        tile.entities.get_mut(position.stack_pos? as usize)
    }
}
