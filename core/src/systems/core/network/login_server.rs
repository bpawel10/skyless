use super::super::attributes::Player;
use super::{
    definitions::{CharacterListEntry, Client, Packet},
    payload::Payload,
    protocol::{Reader, Writer},
};
use crate::core::prelude::*;
use futures::{
    sink::SinkExt,
    stream::{unfold, StreamExt},
};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

system! {
    #[event]
    pub struct LoginEvent(Player, Client);

    const CHANNEL_BUFFER_SIZE: usize = 100;
    const IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
    const LOGIN_PORT: u16 = 7171;
    const GAME_PORT: u16 = 7172;
    let login_socket = SocketAddrV4::new(IP, LOGIN_PORT);

    task! {
        println!("Login server started");
        let listener = TcpListener::bind(login_socket).await.unwrap();
        while let (stream, _) = listener.accept().await.unwrap() {
            let (reader, writer) = stream.into_split();
            let (sender, mut receiver) = mpsc::channel::<Packet>(CHANNEL_BUFFER_SIZE);
            let mut reader = Reader::new(reader);
            let mut writer = Writer::new(writer);
            if let Some(Packet(Payload::Login { xtea, .. })) = reader.next().await {
                writer.set_xtea(xtea);
                yield Some(Arc::new(LoginEvent(Player(1), Client(sender))) as EventType);
                if let Some(packet) = receiver.recv().await {
                    writer.send(packet).await;
                    writer.close().await;
                }
            }
        }
    }

    #[effect(LoginEvent)]
    fn handle_login(event: EventType, _: GameAttributesType, _: WorldType) -> EffectResultType {
        println!("Handle login");

        let task = Box::pin(unfold(event, |event| async move {
            let LoginEvent(_, Client(client)) = event.as_any().downcast_ref::<LoginEvent>().unwrap();
            let character_list_packet = Packet(
                Payload::CharacterList {
                    motd: "Welcome to Skyless POC!".into(),
                    characters: vec![
                        CharacterListEntry {
                            name: "Test".into(),
                            world: "Skyless".into(),
                            ip: Ipv4Addr::LOCALHOST.into(),
                            port: GAME_PORT,
                        },
                    ],
                    premium_days: 0,
                },
            );
            client.send(character_list_packet).await;
            Some((None, event))
        })) as TaskType;

        Some((Vec::new(), vec![task]))
    }
}
