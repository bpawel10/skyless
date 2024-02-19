use num_enum::IntoPrimitive;

#[derive(Debug, Clone)]
pub enum SkillType {
    Fist = 0,
    Club = 1,
    Sword = 2,
    Axe = 3,
    Distance = 4,
    Shield = 5,
    Fishing = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, IntoPrimitive)]
pub enum InventorySlot {
    // Wherever = 0,
    Head = 1,
    Necklace,
    Backpack,
    Armor,
    Right,
    Left,
    Legs,
    Feet,
    Ring,
    Ammo,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntoPrimitive)]
pub enum Directions {
    North = 0,
    East,
    South,
    West,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast,
    None,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntoPrimitive)]
pub enum Skulls {
    None = 0,
    Yellow,
    Green,
    White,
    Red,
    Black,
    Orange,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntoPrimitive)]
pub enum PartyShields {
    None = 0,
    WhiteYelow = 1,
    WhiteBlue = 2,
    Blue = 3,
    Yellow = 4,
}
