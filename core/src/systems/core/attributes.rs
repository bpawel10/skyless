use crate::core::prelude::*;
use crate::systems::prelude::*;
use std::time::Instant;

#[attribute]
pub struct Action(pub Actions);

#[attribute]
pub struct Item(pub u16);

#[attribute]
pub struct Name(pub String);

#[attribute]
#[derive(PartialEq, Eq, Hash)]
pub struct Player(pub u32);

#[attribute]
pub struct LightInfo {
    pub level: u8,
    pub color: u8,
}

#[attribute]
pub struct Health {
    pub value: u16,
    pub max: u16,
}

#[attribute]
pub struct Direction(pub Directions);

impl Direction {
    pub fn apply_to_position(&self, position: Position) -> Position {
        let position_clone = position.clone();
        match self.0 {
            Directions::North => position_clone.y(position.y - 1),
            Directions::East => position_clone.x(position.x + 1),
            Directions::South => position_clone.y(position.y + 1),
            Directions::West => position_clone.x(position.x - 1),
            Directions::SouthWest => position_clone.y(position.y + 1).x(position.x - 1),
            Directions::SouthEast => position_clone.y(position.y + 1).x(position.x + 1),
            Directions::NorthWest => position_clone.y(position.y - 1).x(position.x - 1),
            Directions::NorthEast => position_clone.y(position.y - 1).x(position.x + 1),
            _ => position_clone,
        }
    }

    pub fn between_positions(first: Position, second: Position) -> Option<Self> {
        match first.diff(second) {
            (-1, -1, 0) => Some(Direction(Directions::NorthWest)),
            (0, -1, 0) => Some(Direction(Directions::North)),
            (1, -1, 0) => Some(Direction(Directions::NorthEast)),
            (-1, 0, 0) => Some(Direction(Directions::West)),
            (0, 0, 0) => Some(Direction(Directions::None)),
            (1, 0, 0) => Some(Direction(Directions::East)),
            (-1, 1, 0) => Some(Direction(Directions::SouthWest)),
            (0, 1, 0) => Some(Direction(Directions::South)),
            (1, 1, 0) => Some(Direction(Directions::SouthEast)),
            _ => None,
        }
    }
}

#[attribute]
pub struct Outfit {
    pub r#type: u16,
    pub head: u8,
    pub body: u8,
    pub legs: u8,
    pub feet: u8,
    pub addons: u8,
}

#[attribute]
pub struct Speed(pub u16);

#[attribute]
pub struct Skull(pub Skulls);

#[attribute]
pub struct PartyShield(pub PartyShields);

#[attribute]
pub struct Walking {
    pub until: Instant,
}
