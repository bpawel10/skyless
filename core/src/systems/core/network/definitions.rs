use super::super::definitions::SkillType;
use super::payload::Payload;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tokio::sync::mpsc::Sender;

// #[attribute]
// #[derive(Send)]
#[derive(Debug, Clone)]
pub struct Client(pub Sender<Packet>);

#[derive(Debug)]
pub struct Packet(pub Payload);

#[repr(u8)]
#[derive(TryFromPrimitive)]
pub enum ClientOpcodes {
    Login = 0x01,
    GameLogin = 0x0A,
    Ping = 0x1E,
    PingBack = 0x1D,
    MoveNorth = 0x65,
    MoveEast = 0x66,
    MoveSouth = 0x67,
    MoveWest = 0x68,
    UseItem = 0x82,
    // UseItemOn = 0x83,
}

#[repr(u8)]
#[derive(IntoPrimitive)]
pub enum ServerOpcodes {
    CharacterList = 0x14,
    GameLogin = 0x0A,
    Ping = 0x1E,
    MapFull = 0x64,
    MapNorth = 0x65,
    MapEast = 0x66,
    MapSouth = 0x67,
    MapWest = 0x68,
    InventoryItem = 0x78,
    InventoryEmpty = 0x79,
    Stats = 0xA0,
    Skills = 0xA1,
    WorldLight = 0x82,
    CreatureLight = 0x8D,
    Vip = 0xD2,
    Icons = 0xA2,
    AddCreatureUnknown = 0x61,
    AddCreatureKnown = 0x62,
    ChangedEntity = 0x6B,
    MovedEntity = 0x6D,
}

#[derive(Debug, Clone)]
pub struct Opcode(pub u8);

#[derive(Debug, Clone)]
pub enum AccountName {
    Old(u32),
    // New(String),
}

#[derive(Debug, Clone)]
pub struct CharacterListEntry {
    pub name: String,
    pub world: String,
    pub ip: u32,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub r#type: SkillType,
    pub level: u8,
    pub progress: u8,
}
