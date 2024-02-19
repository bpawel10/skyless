use super::super::attributes::{Item, Player};
use super::{
    definitions::{AccountName, ClientOpcodes, Packet},
    payload::Payload,
    rsa::Rsa,
    xtea::Xtea,
};
use crate::prelude::*;
use bytes::{Buf, BufMut, BytesMut};
use futures::{
    sink::Sink,
    stream::Stream,
    task::{Context, Poll},
};
use skyless_core::prelude::*;
use std::{convert::TryInto, pin::Pin};
use tokio::{
    io::{Error, Result},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};
use tokio_util::codec::{Decoder, Encoder, FramedRead, FramedWrite};

#[derive(Debug)]
pub struct Protocol {
    rsa: Rsa,
    xtea: Option<Xtea>,
}

impl Protocol {
    pub fn new() -> Self {
        Self {
            rsa: Rsa::new(),
            xtea: None,
        }
    }

    pub fn set_xtea(&mut self, xtea: Xtea) {
        self.xtea = Some(xtea);
    }
}

impl Decoder for Protocol {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Packet>> {
        if buffer.len() <= 2 {
            return Ok(None);
        }

        let mut cloned = buffer.clone();
        let length = cloned.get_u16_le() as usize;

        if length == 0 || buffer.len() < length + 2 {
            return Ok(None);
        }

        buffer.advance(2);
        let mut msg = buffer.split_to(length);

        let (opcode, mut msg) = match self.xtea {
            Some(xtea) => {
                let mut decrypted = BytesMut::from(xtea.decrypt(msg.to_vec()).as_slice());
                let decrypted_length = decrypted.get_u16_le();
                let mut truncated = decrypted.split_to(decrypted_length as usize);
                let opcode = truncated.get_u8();
                (opcode, truncated)
            }
            None => (msg.get_u8(), msg.split()),
        };

        let payload = match opcode.try_into().ok() {
            Some(ClientOpcodes::Login) => {
                msg.advance(1);
                let mut version = [0; 2];
                msg.copy_to_slice(&mut version);
                let version =
                    u16::from_str_radix(&format!("{}{}", version[1], version[0]), 16).unwrap();
                msg.advance(13);
                let decrypted = self.rsa.decrypt(msg.chunk())?;
                let mut msg = BytesMut::from(decrypted.as_slice());
                msg.advance(1);
                let xtea = Xtea::new([
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                ]);
                let name = msg.get_u32_le(); // TODO: handle name string (8.3+)
                                             // TODO: add a way to parse string directly (something like 'msg.get_string')
                let password_length = msg.get_u16_le();
                let password =
                    String::from_utf8_lossy(msg.copy_to_bytes(password_length.into()).chunk())
                        .to_string();
                Payload::Login {
                    version,
                    xtea,
                    name: AccountName::Old(name),
                    password,
                }
            }
            Some(ClientOpcodes::GameLogin) => {
                msg.advance(1);
                let mut version = [0; 2];
                msg.copy_to_slice(&mut version);
                let version =
                    u16::from_str_radix(&format!("{}{}", version[1], version[0]), 16).unwrap();
                msg.advance(1);
                let decrypted = self.rsa.decrypt(msg.chunk()).unwrap();
                let mut msg = BytesMut::from(decrypted.as_slice());
                msg.advance(1);
                let xtea = Xtea::new([
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                    msg.get_u32_le(),
                ]);
                msg.advance(1);
                let name = msg.get_u32_le(); // TODO: handle name string (8.3+)
                let character_length = msg.get_u16_le();
                let character =
                    String::from_utf8_lossy(msg.copy_to_bytes(character_length.into()).chunk())
                        .to_string();
                let password_length = msg.get_u16_le();
                let password =
                    String::from_utf8_lossy(msg.copy_to_bytes(password_length.into()).chunk())
                        .to_string();
                Payload::GameLogin {
                    version,
                    xtea,
                    name: AccountName::Old(name),
                    character,
                    password,
                }
            }
            Some(ClientOpcodes::Ping) => Payload::Ping,
            Some(ClientOpcodes::PingBack) => Payload::PingBack,
            Some(ClientOpcodes::MoveNorth) => Payload::Move {
                player: Player(1),
                direction: Direction(Directions::North),
            },
            Some(ClientOpcodes::MoveEast) => Payload::Move {
                player: Player(1),
                direction: Direction(Directions::East),
            },
            Some(ClientOpcodes::MoveSouth) => Payload::Move {
                player: Player(1),
                direction: Direction(Directions::South),
            },
            Some(ClientOpcodes::MoveWest) => Payload::Move {
                player: Player(1),
                direction: Direction(Directions::West),
            },
            Some(ClientOpcodes::UseItem) => {
                let x = msg.get_u16_le();
                let y = msg.get_u16_le();
                let z = msg.get_u8();
                let id = msg.get_u16_le();
                let stack_pos = msg.get_u8();
                let index = msg.get_u8();
                Payload::UseItem {
                    position: Position {
                        x,
                        y,
                        z,
                        stack_pos: Some(stack_pos.into()),
                    },
                    item: Item(id),
                }
            }
            _ => Payload::Raw(msg),
        };

        Ok(Some(Packet(payload)))
    }
}

impl Encoder<Packet> for Protocol {
    type Error = Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<()> {
        let Packet(payload) = item;
        let buffer = payload.to_bytes();
        let msg2 = match self.xtea {
            Some(xtea) => {
                let mut msg2 = BytesMut::new();
                msg2.put_u16_le(buffer.len() as u16);
                msg2.put_slice(&buffer);
                xtea.encrypt(msg2.to_vec())
            }
            None => buffer,
        };
        dst.put_u16_le(msg2.len() as u16);
        dst.put_slice(&msg2.to_vec());
        Ok(())
    }
}

#[derive(Debug)]
pub struct Reader {
    framed: FramedRead<OwnedReadHalf, Protocol>,
}

impl Reader {
    pub fn new(stream: OwnedReadHalf) -> Self {
        let framed = FramedRead::new(stream, Protocol::new());
        Self { framed }
    }

    pub fn set_xtea(&mut self, xtea: Xtea) {
        self.framed.decoder_mut().set_xtea(xtea);
    }
}

impl Stream for Reader {
    type Item = Packet;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.framed).poll_next(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(result.ok()),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug)]
pub struct Writer {
    framed: FramedWrite<OwnedWriteHalf, Protocol>,
}

impl Writer {
    pub fn new(stream: OwnedWriteHalf) -> Self {
        let framed = FramedWrite::new(stream, Protocol::new());
        Self { framed }
    }

    pub fn set_xtea(&mut self, xtea: Xtea) {
        self.framed.encoder_mut().set_xtea(xtea);
    }
}

impl Sink<Packet> for Writer {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.framed).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Packet) -> Result<()> {
        Pin::new(&mut self.framed).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.framed).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.framed).poll_close(cx)
    }
}
