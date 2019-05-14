use std::net::{SocketAddr, TcpStream};
use std::thread;
use uuid::Uuid;
use websocket::sender::Writer;
use websocket::sync::Server;
use websocket::OwnedMessage;

use crate::communication::{IncomingMessage, IncomingMessageType};
use crate::map::send_map;
use crate::players::{add_player, move_player, send_all_players, send_new_player, send_player};

static SERVER_IP: &str = "0.0.0.0:2794";

fn handle_game_message(id: Uuid, message: &OwnedMessage, sender: &mut Writer<TcpStream>) {
    if let OwnedMessage::Text(str_message) = message {
        if let Ok(message) = serde_json::from_str::<IncomingMessage<serde_json::Value>>(str_message)
        {
            match message._type {
                IncomingMessageType::Map => send_map(sender).expect("Could not send map"),
                IncomingMessageType::NewPlayer => {
                    if let Ok(player) = add_player(id, message.payload) {
                        send_map(sender).expect("Could not send map");
                        send_new_player(sender, player).expect("Could not send coords");
                        send_all_players(sender).expect("Could not send coords")
                    }
                }
                IncomingMessageType::PlayerCoords => {
                    if let Ok(player) = move_player(id, message.payload) {
                        send_player(sender, player).expect("Could not send coords");
                    }
                }
            }
        }
    }
}

fn handle_message(id: Uuid, message: OwnedMessage, sender: &mut Writer<TcpStream>, ip: SocketAddr) {
    match message {
        OwnedMessage::Close(_) => {
            sender
                .send_message(&OwnedMessage::Close(None))
                .expect("Could not send close message");
            info!("Client {} disconnected", ip);
            return;
        }
        OwnedMessage::Ping(ping) => {
            sender
                .send_message(&OwnedMessage::Pong(ping))
                .expect("Could not send pong message");
        }
        _ => handle_game_message(id, &message, sender),
    }
}

pub fn launch_server() {
    let server = Server::bind(SERVER_IP).expect("Could not bind server");

    info!("Server launched at {}", SERVER_IP);

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let client = request.accept().expect("Could not accept client");
            let ip = client.peer_addr().expect("Could not get client IP");

            info!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().expect("Could not split client");
            let id = Uuid::new_v4();

            for message in receiver.incoming_messages() {
                let message = message.expect("Could not get incoming message");

                debug!("Received message: {:?}", message);
                handle_message(id, message, &mut sender, ip);
            }
        });
    }
}
