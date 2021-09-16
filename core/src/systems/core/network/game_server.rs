use super::{
    definitions::{Client, Packet, SkillEntry},
    events::*,
    payload::Payload,
    protocol::{Reader, Writer},
    xtea::Xtea,
};
use crate::core::prelude::*;
use crate::systems::prelude::*;
use futures::{
    sink::SinkExt,
    stream::{unfold, StreamExt},
    FutureExt,
};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

system! {
    #[attribute]
    pub struct Clients(pub HashMap<Player, Client>);

    #[event]
    pub struct GameConnectionEvent {
        pub reader: Receiver<Packet>,
        pub writer: Sender<Packet>,
    }

    #[event]
    pub struct GameLoginEvent(pub Player, pub Client);

    #[event]
    pub struct GamePayloadEvent(pub Payload);

    #[effect(SystemsLoadedEvent)]
    fn init_players_attribute(_: EventType, _: GameAttributesType, _: WorldType) -> EffectResultType {
        let mut commands = Vec::new();
        commands.push(Box::new(SetGameAttributeCommand(Box::new(Clients(HashMap::new())))) as CommandType);
        Some((commands, Vec::new()))
    }

    const CHANNEL_BUFFER_SIZE: usize = 100;
    const IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
    const GAME_PORT: u16 = 7172;
    let game_socket = SocketAddrV4::new(IP, GAME_PORT);

    task! {
        println!("Game server started");
        let listener = TcpListener::bind(game_socket).await.unwrap();
        while let (connection, _) = listener.accept().await.unwrap() {
            let (reader, writer) = connection.into_split();
            let reader_tcp = Reader::new(reader);
            let mut writer_tcp = Writer::new(writer);
            let (writer, mut receiver) = mpsc::channel::<Packet>(CHANNEL_BUFFER_SIZE);
            let (sender_xtea, receiver_xtea) = oneshot::channel::<Xtea>();

            tasker.send(Box::pin(async move {
                if let Ok(xtea) = receiver_xtea.await {
                    writer_tcp.set_xtea(xtea);
                    while let Some(packet) = receiver.recv().await {
                        writer_tcp.send(packet).await;
                    }
                }
                None
            }.into_stream()) as TaskType).await;

            tasker.send(Box::pin(unfold((Some(sender_xtea), reader_tcp, writer), |(sender_xtea, mut reader_tcp, writer)| async move {
                if let Some(Packet(payload)) = reader_tcp.next().await {
                    let event = match payload {
                        Payload::GameLogin { xtea, .. } => {
                            reader_tcp.set_xtea(xtea);
                            if let Some(sender_xtea) = sender_xtea {
                                sender_xtea.send(xtea).unwrap();
                            }
                            Arc::new(GameLoginEvent(Player(1), Client(writer.clone()))) as EventType
                        },
                        _ => Arc::new(GamePayloadEvent(payload)) as EventType
                    };
                    Some((Some(event), (None, reader_tcp, writer)))
                } else {
                    Some((None, (None, reader_tcp, writer)))
                }
            })) as TaskType).await;
        }
        yield None;
    }

    #[effect(GameLoginEvent)]
    fn handle_game_login(event: EventType, attributes: GameAttributesType, world: WorldType) -> EffectResultType {
        println!("Handle game login");

        let GameLoginEvent(player, Client(client)) = event.as_any().downcast_ref::<GameLoginEvent>().unwrap();
        let client_clone = client.clone();
        let mut commands = Vec::new();
        let mut tasks = Vec::new();
        let attributes = attributes.lock().unwrap();
        if let Some(clients) = attributes.clients() {
            let mut clients_new = clients.0.clone();
            clients_new.insert(player.clone(), Client(client_clone.clone()));
            commands.push(Box::new(SetGameAttributeCommand(Box::new(Clients(clients_new)))) as CommandType);
        }

        let player = entity![
            player.clone(),
            Name("Skyless".into()),
            Health { value: 150, max: 150 },
            Direction(Directions::South),
            Outfit { r#type: 128, head: 78, body: 69, legs: 58, feet: 76, addons: 0 },
            LightInfo { level: 0xFF, color: 0x00 },
            Speed(220),
            Skull(Skulls::None),
            PartyShield(PartyShields::None)
        ];
        let player_id = player.player().unwrap().0;

        commands.push(Box::new(AddEntityCommand { position: Position(128, 128, 7), entity: player }) as CommandType);

        let player_login_payload = Payload::PlayerLogin {
            id: player_id,
            beat: 50,
            can_report_bugs: false,
        };
        let map_payload = Payload::MapFull {
            position: Position(128, 128, 7),
            world,
        };
        let mut inventory_items_payloads = vec![
            Payload::InventoryItem { slot: InventorySlot::Head, item: None },
            Payload::InventoryItem { slot: InventorySlot::Necklace, item: None },
            Payload::InventoryItem { slot: InventorySlot::Backpack, item: None },
            Payload::InventoryItem { slot: InventorySlot::Armor, item: None },
            Payload::InventoryItem { slot: InventorySlot::Right, item: None },
            Payload::InventoryItem { slot: InventorySlot::Left, item: None },
            Payload::InventoryItem { slot: InventorySlot::Legs, item: None },
            Payload::InventoryItem { slot: InventorySlot::Feet, item: None },
            Payload::InventoryItem { slot: InventorySlot::Ring, item: None },
            Payload::InventoryItem { slot: InventorySlot::Ammo, item: None },
        ];
        let stats_payload = Payload::Stats {
            health: 150,
            health_max: 150,
            capacity: 400,
            experience: 0,
            level: 1,
            level_progress: 0,
            mana: 0,
            mana_max: 0,
            magic: 0,
            magic_progress: 0,
            soul: 100,
            stamina: 56 * 60,
        };
        let skills_payload = Payload::Skills(vec![
            SkillEntry { r#type: SkillType::Fist, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Club, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Sword, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Axe, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Distance, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Shield, level: 10, progress: 0 },
            SkillEntry { r#type: SkillType::Fishing, level: 10, progress: 0 },
        ]);
        let world_light_payload = Payload::WorldLight(LightInfo { level: 0xFF, color: 0x00 });
        let creature_light_payload = Payload::CreatureLight {
            player: Player(player_id),
            light: LightInfo { level: 0xFF, color: 0x00 },
        };
        let icons_payload = Payload::Icons(0x00);

        let mut payloads = vec![player_login_payload, map_payload];
        payloads.append(&mut inventory_items_payloads);
        payloads.append(&mut vec![
            stats_payload,
            skills_payload,
            world_light_payload,
            creature_light_payload,
            icons_payload,
        ]);
        let packet = Packet(Payload::Combined(payloads));

        tasks.push(Box::pin(async move {
            client_clone.send(packet).await;
            None
        }.into_stream()) as TaskType);

        Some((commands, tasks))
    }

    #[effect(GamePayloadEvent)]
    fn handle_game_payload(event: EventType, _: GameAttributesType, _: WorldType) -> EffectResultType {
        let tasks = vec![
            Box::pin(async move {
                let GamePayloadEvent(payload) = event.as_any().downcast_ref::<GamePayloadEvent>().unwrap();
                match payload {
                    Payload::Ping => Some(Arc::new(PingPayloadEvent) as EventType),
                    Payload::PingBack => Some(Arc::new(PingBackPayloadEvent) as EventType),
                    Payload::Move { player, direction } =>
                        Some(Arc::new(MovePayloadEvent {
                            player: player.clone(),
                            direction: direction.clone(),
                        }) as EventType),
                    Payload::UseItem { position, item } =>
                        Some(Arc::new(UseItemPayloadEvent {
                            position: position.clone(),
                            item: item.clone(),
                        }) as EventType),
                    _ => None,
                }
            }.into_stream()) as TaskType,
        ];
        Some((Vec::new(), tasks))
    }

    #[effect(ChangedEntityEvent)]
    fn handle_changed_entity(event: EventType, attributes: GameAttributesType, world: WorldType) -> EffectResultType {
        let ChangedEntityEvent { position, attribute_name } = event.as_any().downcast_ref::<ChangedEntityEvent>().unwrap();
        let mut tasks = Vec::new();
        if attribute_name.as_str() == "item" { // TODO: somehow check attribute type instead of name
            let game_attributes = attributes.lock().unwrap();
            if let Some(mut tasks2) = (|| {
                let clients = game_attributes.clients()?;
                let world = world.lock().unwrap();
                let entity = world.entity(&position)?;
                let item = entity.item()?;
                let mut tasks = Vec::new();
                for Client(client) in clients.0.values() {
                    let client = client.clone();
                    let payload = Payload::ChangedEntity { position: position.clone(), item: item.clone() };
                    tasks.push(Box::pin(async move {
                        client.send(Packet(payload)).await;
                        None
                    }.into_stream()) as TaskType);
                }
                Some(tasks)
            })() {
                tasks.append(&mut tasks2);
            }
        }
        Some((Vec::new(), tasks))
    }
}
