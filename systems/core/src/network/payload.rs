use super::definitions::*;
use super::xtea::Xtea;
use crate::prelude::*;
use bytes::{BufMut, BytesMut};
use itertools::Itertools;
use skyless_core::prelude::*;

const VIEWPORT_X: u16 = 8;
const VIEWPORT_Y: u16 = 6;

#[derive(Clone)]
pub enum Payload {
    Raw(BytesMut),
    Combined(Vec<Payload>),
    Login {
        version: u16,
        xtea: Xtea,
        name: AccountName,
        password: String,
    },
    CharacterList {
        motd: String,
        characters: Vec<CharacterListEntry>,
        premium_days: u16,
    },
    GameLogin {
        version: u16,
        xtea: Xtea,
        name: AccountName,
        character: String,
        password: String,
    },
    Ping,
    PingBack,
    PlayerLogin {
        id: u32,
        beat: u16,
        can_report_bugs: bool,
    },
    Map {
        from: Position,
        to: Position,
        position_z: u8,
        world: WorldType,
    },
    MapFull {
        position: Position,
        world: WorldType,
    },
    MapNorth {
        position: Position,
        world: WorldType,
    },
    MapWest {
        position: Position,
        world: WorldType,
    },
    MapEast {
        position: Position,
        world: WorldType,
    },
    MapSouth {
        position: Position,
        world: WorldType,
    },
    InventoryItem {
        slot: InventorySlot,
        item: Option<Item>,
    },
    Stats {
        health: u16,
        health_max: u16,
        capacity: u16,
        experience: u32,
        level: u16,
        level_progress: u8,
        mana: u16,
        mana_max: u16,
        magic: u8,
        magic_progress: u8,
        soul: u8,
        stamina: u16,
    },
    Skills(Vec<SkillEntry>),
    WorldLight(LightInfo),
    CreatureLight {
        player: Player,
        light: LightInfo,
    },
    Icons(u16), // TODO: use Vec of conditions instead?
    Move {
        player: Player,
        direction: Direction,
    },
    MovedEntity {
        from: Position,
        to: Position,
    },
    ChangedEntity {
        position: Position,
        item: Item,
    },
    UseItem {
        position: Position,
        item: Item,
    },
}

impl Payload {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::Combined(payloads) => {
                let mut msg = BytesMut::new();
                for payload in payloads.into_iter() {
                    msg.put_slice(&payload.to_bytes());
                }
                msg.to_vec()
            }
            Self::CharacterList {
                motd,
                characters,
                premium_days,
            } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::CharacterList.into());
                msg.put_u16_le(motd.len() as u16);
                msg.put_slice(motd.as_bytes());
                msg.put_u8(0x64);
                msg.put_u8(characters.len() as u8);
                for character in characters.iter() {
                    msg.put_u16_le(character.name.len() as u16);
                    msg.put_slice(character.name.as_bytes());
                    msg.put_u16_le(character.world.len() as u16);
                    msg.put_slice(character.world.as_bytes());
                    msg.put_u32(character.ip);
                    msg.put_u16_le(character.port);
                }
                msg.put_u16_le(premium_days);
                msg.to_vec()
            }
            Self::PlayerLogin {
                id,
                beat,
                can_report_bugs,
            } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::GameLogin.into());
                msg.put_u32(id);
                msg.put_u16_le(beat);
                msg.put_u8(can_report_bugs as u8);
                msg.to_vec()
            }
            Self::Ping => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::Ping.into());
                msg.to_vec()
            }
            Self::Map {
                from,
                to,
                position_z,
                world,
            } => {
                let mut msg = BytesMut::new();
                let x_range = from.x..=to.x;
                let y_range = from.y..=to.y;
                let x_y_range = x_range.cartesian_product(y_range);
                // TODO: different z_range when z > 7
                let z_range = (to.z..=from.z).rev();
                let world = world.lock().unwrap();
                let mut skip: i16 = -1;
                for z in z_range {
                    let offset = u16::from(position_z - z);
                    for (x, y) in x_y_range.clone() {
                        match world.0.get(&Position(x + offset, y + offset, z)) {
                            Some(tile) => {
                                if skip >= 0 {
                                    msg.put_u8(skip as u8);
                                    msg.put_u8(0xFF);
                                }
                                skip = 0;
                                for entity in tile.entities.iter() {
                                    if let Some(Player(id)) = entity.player() {
                                        // TODO: handle known creature
                                        msg.put_u16_le(0x61);
                                        msg.put_u32_le(0x00);
                                        msg.put_u32(*id);
                                        let Name(name) = entity.name().unwrap();
                                        msg.put_u16_le(name.len() as u16);
                                        msg.put_slice(name.as_bytes());
                                        // TODO: handle creature health hidden case
                                        let Health { value, max } = entity.health().unwrap();
                                        let health_percentage = *value as f64 / *max as f64;
                                        msg.put_u8(health_percentage.ceil() as u8 * 100);
                                        let Direction(direction) = entity.direction().unwrap();
                                        msg.put_u8(*direction as u8);
                                        let Outfit {
                                            r#type,
                                            head,
                                            body,
                                            legs,
                                            feet,
                                            addons,
                                        } = entity.outfit().unwrap();
                                        msg.put_u16_le(*r#type);
                                        msg.put_u8(*head);
                                        msg.put_u8(*body);
                                        msg.put_u8(*legs);
                                        msg.put_u8(*feet);
                                        msg.put_u8(*addons);
                                        let LightInfo { level, color } =
                                            entity.light_info().unwrap();
                                        msg.put_u8(*level); // TODO: send 0xFF when "access player" (gm, god etc.)
                                        msg.put_u8(*color);
                                        let Speed(speed) = entity.speed().unwrap();
                                        msg.put_u16_le(*speed);
                                        let Skull(skull) = entity.skull().unwrap();
                                        msg.put_u8(*skull as u8);
                                        let PartyShield(party_shield) =
                                            entity.party_shield().unwrap();
                                        msg.put_u8(*party_shield as u8);
                                    } else if let Some(Item(id)) = entity.item() {
                                        msg.put_u16_le(*id);
                                    }
                                }
                            }
                            None => {
                                skip += 1;
                                if skip == 0xFF {
                                    msg.put_u8(0xFF);
                                    msg.put_u8(0xFF);
                                    skip = -1;
                                }
                            }
                        };
                    }
                }
                if skip >= 0 {
                    msg.put_u8(skip as u8);
                    msg.put_u8(0xFF);
                }
                msg.to_vec()
            }
            Self::MapFull { position, world } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MapFull.into());
                msg.put_u16_le(position.x);
                msg.put_u16_le(position.y);
                msg.put_u8(position.z);
                let from = Position(position.x - VIEWPORT_X, position.y - VIEWPORT_Y, 7);
                let to = Position(position.x + VIEWPORT_X + 1, position.y + VIEWPORT_Y + 1, 0);
                let map = Payload::Map {
                    from,
                    to,
                    position_z: position.z,
                    world,
                };
                msg.put_slice(&map.to_bytes());
                msg.to_vec()
            }
            Self::MapNorth { position, world } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MapNorth.into());
                let from = Position(position.x - VIEWPORT_X, position.y - VIEWPORT_Y, 7);
                let to = Position(position.x + VIEWPORT_X + 1, from.y, 0);
                msg.put_slice(
                    &Payload::Map {
                        from,
                        to,
                        position_z: position.z,
                        world,
                    }
                    .to_bytes(),
                );
                msg.to_vec()
            }
            Self::MapEast { position, world } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MapEast.into());
                let from = Position(position.x + VIEWPORT_X + 1, position.y - VIEWPORT_Y, 7);
                let to = Position(from.x, position.y + VIEWPORT_Y + 1, 0);
                msg.put_slice(
                    &Payload::Map {
                        from,
                        to,
                        position_z: position.z,
                        world,
                    }
                    .to_bytes(),
                );
                msg.to_vec()
            }
            Self::MapSouth { position, world } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MapSouth.into());
                let from = Position(position.x - VIEWPORT_X, position.y + VIEWPORT_Y + 1, 7);
                let to = Position(position.x + VIEWPORT_X + 1, from.y, 0);
                msg.put_slice(
                    &Payload::Map {
                        from,
                        to,
                        position_z: position.z,
                        world,
                    }
                    .to_bytes(),
                );
                msg.to_vec()
            }
            Self::MapWest { position, world } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MapWest.into());
                let from = Position(position.x - VIEWPORT_X, position.y - VIEWPORT_Y, 7);
                let to = Position(from.x, position.y + VIEWPORT_Y + 1, 0);
                msg.put_slice(
                    &Payload::Map {
                        from,
                        to,
                        position_z: position.z,
                        world,
                    }
                    .to_bytes(),
                );
                msg.to_vec()
            }
            Self::InventoryItem { slot, item } => {
                let mut msg = BytesMut::new();
                match item {
                    Some(Item(id)) => {
                        msg.put_u8(ServerOpcodes::InventoryItem.into());
                        msg.put_u8(slot.into());
                        msg.put_u16_le(id);
                    }
                    None => {
                        msg.put_u8(ServerOpcodes::InventoryEmpty.into());
                        msg.put_u8(slot.into());
                    }
                };
                msg.to_vec()
            }
            Self::Stats {
                health,
                health_max,
                capacity,
                experience,
                level,
                level_progress,
                mana,
                mana_max,
                magic,
                magic_progress,
                soul,
                stamina,
            } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::Stats.into());
                msg.put_u16_le(health);
                msg.put_u16_le(health_max);
                msg.put_u16_le(capacity / 100);
                msg.put_u32_le(experience);
                msg.put_u16_le(level);
                msg.put_u8(level_progress);
                msg.put_u16_le(mana);
                msg.put_u16_le(mana_max);
                msg.put_u8(magic);
                msg.put_u8(magic_progress);
                msg.put_u8(soul);
                msg.put_u16_le(stamina);
                msg.to_vec()
            }
            Self::Skills(skills) => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::Skills.into());
                for SkillEntry {
                    level, progress, ..
                } in skills.into_iter()
                {
                    msg.put_u8(level);
                    msg.put_u8(progress);
                }
                msg.to_vec()
            }
            Self::WorldLight(LightInfo { level, color }) => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::WorldLight.into());
                msg.put_u8(level); // TODO: send 0xFF when "access player" (gm, god etc.)
                msg.put_u8(color);
                msg.to_vec()
            }
            Self::CreatureLight {
                player: Player(id),
                light: LightInfo { level, color },
            } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::CreatureLight.into());
                msg.put_u32_le(id);
                msg.put_u8(level); // TODO: send 0xFF when "access player" (gm, god etc.)
                msg.put_u8(color);
                msg.to_vec()
            }
            Self::Icons(icons) => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::Icons.into());
                msg.put_u16(icons);
                msg.to_vec()
            }
            Self::ChangedEntity { position, item } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::ChangedEntity.into());
                msg.put_u16_le(position.x);
                msg.put_u16_le(position.y);
                msg.put_u8(position.z);
                msg.put_u8(position.stack_pos.unwrap() as u8);
                msg.put_u16_le(item.0);
                msg.to_vec()
            }
            Self::MovedEntity { from, to } => {
                let mut msg = BytesMut::new();
                msg.put_u8(ServerOpcodes::MovedEntity.into());
                msg.put_u16_le(from.x);
                msg.put_u16_le(from.y);
                msg.put_u8(from.z);
                msg.put_u8(from.stack_pos.unwrap() as u8);
                msg.put_u16_le(to.x);
                msg.put_u16_le(to.y);
                msg.put_u8(to.z);
                msg.to_vec()
            }
            // Self::Raw(msg) => {
            //     let mut msg2 = BytesMut::new();
            //     msg2.put_u8(opcode);
            //     msg2.put_slice(&msg.to_vec());
            //     msg2.to_vec()
            // },
            _ => Vec::new(), // TODO: error handling
        }
    }
}
