#![forbid(unsafe_code)]

mod core;
mod systems;

use crate::core::{prelude::*, CommandType, EmitEventCommand, TaskType};
use std::io::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

// TODO: check what buffer size is the best
const CHANNEL_BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    let (commands_sender, mut commands_receiver) =
        mpsc::channel::<CommandType>(CHANNEL_BUFFER_SIZE);
    let (tasks_sender, mut tasks_receiver) = mpsc::channel::<TaskType>(CHANNEL_BUFFER_SIZE);
    let commands_sender_clone = commands_sender.clone();

    tokio::spawn(async move {
        while let Some(mut task) = tasks_receiver.recv().await {
            let commands_sender_clone = commands_sender_clone.clone();
            tokio::spawn(async move {
                while let Some(Some(event)) = task.next().await {
                    let command = EmitEventCommand(event);
                    commands_sender_clone.send(Box::new(command)).await.unwrap();
                }
            });
        }
    });

    let game = Arc::new(Mutex::new(Game::new(tasks_sender.clone())));

    println!("Loading systems");

    // TODO: load systems dynamically based on config file
    systems::core::map::hardcoded::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    systems::core::network::login_server::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    systems::core::network::game_server::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    // systems::core::network::payloads::ping::system(game.clone(), tasks_sender.clone())
    //     .await
    //     .unwrap();
    systems::core::network::handlers::r#move::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    systems::core::network::handlers::use_item::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    systems::scripts::lever::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    systems::scripts::switch::system(game.clone(), tasks_sender.clone())
        .await
        .unwrap();
    // systems::scripts::tick::system(game.clone(), tasks_sender.clone())
    //     .await
    //     .unwrap();

    commands_sender
        .send(Box::new(EmitEventCommand(Arc::new(SystemsLoadedEvent))))
        .await;

    println!("Game started");

    while let Some(command) = commands_receiver.recv().await {
        let mut g = game.lock().unwrap();
        g.process(command).await;
    }

    Ok(())
}
