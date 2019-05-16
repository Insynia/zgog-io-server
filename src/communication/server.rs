use std::thread;
use uuid::Uuid;
use websocket::sync::Server;
use websocket::OwnedMessage;

use crate::communication::client::{
    add_client, remove_client, send_all_clients, with_client_id, Socket,
};
use crate::communication::{IncomingMessage, IncomingMessageType, OutgoingMessageType};
use crate::map::send_map;
use crate::players::{add_player, move_player, send_all_players, send_hero};

static SERVER_IP: &str = "0.0.0.0:2794";

fn handle_game_message(id: Uuid, message: &OwnedMessage) {
    if let OwnedMessage::Text(str_message) = message {
        if let Ok(message) = serde_json::from_str::<IncomingMessage<serde_json::Value>>(str_message)
        {
            match message._type {
                IncomingMessageType::Map => with_client_id(id, &|s: &mut Socket| {
                    send_map(s).expect("Could not send map")
                }),
                IncomingMessageType::NewPlayer => {
                    if let Ok(player) = add_player(id, message.payload) {
                        with_client_id(id, &|s: &mut Socket| {
                            send_map(s).expect("Could not send map");
                            send_hero(s, player.clone()).expect("Could not send coords");
                            send_all_players(s).expect("Could not send coords")
                        });
                        send_all_clients(OutgoingMessageType::NewPlayer, Some(player));
                    }
                }
                IncomingMessageType::PlayerCoords => {
                    if let Ok(player) = move_player(id, message.payload) {
                        send_all_clients(OutgoingMessageType::PlayerUpdated, Some(player));
                    }
                }
            }
        }
    }
}

fn handle_message(id: Uuid, message: OwnedMessage) {
    match message {
        // TODO send player disconnected to clients
        OwnedMessage::Close(_) => {
            remove_client(id);
            info!("Client {} disconnected", id);
            return;
        }
        OwnedMessage::Ping(ping) => {
            with_client_id(id, &|s: &mut Socket| {
                s.send_message(&OwnedMessage::Pong(ping.clone()))
                    .expect("Could not send pong message")
            });
        }
        _ => handle_game_message(id, &message),
    }
}

/// Launches the game server.
pub fn launch_server() {
    let server = Server::bind(SERVER_IP).expect("Could not bind server");

    info!("Server launched at {}", SERVER_IP);

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let client = request.accept().expect("Could not accept client");
            let id = Uuid::new_v4();

            info!("Connection from {}", id);

            let (mut receiver, sender) = client.split().expect("Could not split client");

            add_client(id, sender);

            for message in receiver.incoming_messages() {
                let message = message.expect("Could not get incoming message");

                debug!("Received message: {:?}", message);
                handle_message(id, message);
            }
        });
    }
}
