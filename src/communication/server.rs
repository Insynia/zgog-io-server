use std::thread;
use uuid::Uuid;
use websocket::sync::Server;
use websocket::OwnedMessage;

use crate::communication::client::{
    add_client, remove_client, send_all_clients, with_client_id, Socket,
};
use crate::communication::{IncomingMessage, IncomingMessageType, OutgoingMessageType};
use crate::map::send_map;
use crate::player::{add_player, move_player, remove_player, send_all_players, send_hero, PLAYERS};

static SERVER_IP: &str = "0.0.0.0:2794";

fn handle_game_message(id: Uuid, message: &OwnedMessage) -> Result<(), String> {
    if let OwnedMessage::Text(str_message) = message {
        if let Ok(message) = serde_json::from_str::<IncomingMessage<serde_json::Value>>(str_message)
        {
            match message._type {
                IncomingMessageType::Map => {
                    with_client_id(id, &|s: &mut Socket| send_map(s))?;
                }
                IncomingMessageType::NewPlayer => {
                    let player = add_player(id, message.payload)?;

                    with_client_id(id, &|s: &mut Socket| {
                        send_map(s)?;
                        send_hero(s, player.clone())?;
                        send_all_players(s)?;
                        Ok(())
                    })?;
                    send_all_clients(OutgoingMessageType::NewPlayer, Some(player));
                }
                IncomingMessageType::PlayerCoords => {
                    move_player(id, message.payload)?;
                }
            };
        }
    }
    Ok(())
}

fn handle_message(id: Uuid, message: OwnedMessage) {
    match message {
        OwnedMessage::Close(_) => {
            remove_player(id);
            remove_client(id);
        }
        _ => {
            if let Err(err) = handle_game_message(id, &message) {
                error!("Error handling game message: {}", err);
            }
        }
    }
}

/// Delay to send coordinates of players in ms
static SEND_COORDS_DELAY: u64 = 100;

/// Launches the game server.
pub fn launch_server() {
    let server = Server::bind(SERVER_IP).expect("Could not bind server");

    info!("Server launched at {}", SERVER_IP);

    thread::spawn(|| {
        use crossbeam_channel::tick;
        use std::time::Duration;

        let ticker = tick(Duration::from_millis(SEND_COORDS_DELAY));

        loop {
            ticker.recv().unwrap();
            send_all_clients(
                OutgoingMessageType::AllPlayers,
                Some(
                    PLAYERS
                        .read()
                        .expect("Could not lock players mutex")
                        .clone(),
                ),
            );
        }
    });

    for new_client in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let client = new_client.accept().expect("Could not accept client");
            let id = Uuid::new_v4();

            info!("Connection from {}", id);

            let (mut receiver, sender) = client.split().expect("Could not split client");

            add_client(id, sender);

            for message in receiver.incoming_messages().filter_map(Result::ok) {
                debug!("Received message: {:?}", message);
                handle_message(id, message);
            }
        });
    }
}
