#![feature(custom_attribute)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::net::TcpStream;
use std::thread;
use websocket::sender::Writer;
use websocket::sync::Server;
use websocket::OwnedMessage;

mod communication;
mod map;
mod players;

use crate::communication::{Message, MessageType};
use crate::map::MAP;
use crate::players::{add_player, move_player, PLAYERS};

fn handle_message(
    id: Option<usize>,
    message: &OwnedMessage,
    sender: &mut Writer<TcpStream>,
) -> Option<usize> {
    println!("{:?}", message);

    if let OwnedMessage::Text(str_message) = message {
        if let Ok(message) = serde_json::from_str::<Message<serde_json::Value>>(str_message) {
            match message._type {
                MessageType::Map => {
                    sender
                        .send_message(&OwnedMessage::Text(
                            Message {
                                _type: MessageType::Map,
                                payload: Some(MAP.clone()),
                            }
                            .into(),
                        ))
                        .expect("Could not send map");
                    sender
                        .send_message(&OwnedMessage::Text(
                            Message {
                                _type: MessageType::PlayerCoords,
                                payload: Some(PLAYERS.lock().unwrap().clone()),
                            }
                            .into(),
                        ))
                        .expect("Could not send coords");
                }
                MessageType::NewPlayer => {
                    if let Ok(player) = add_player(message.payload) {
                        sender
                            .send_message(&OwnedMessage::Text(
                                Message {
                                    _type: MessageType::NewPlayer,
                                    payload: Some(player.clone()),
                                }
                                .into(),
                            ))
                            .expect("Could not send coords");
                        return Some(player.id);
                    }
                }
                MessageType::PlayerCoords => {
                    if let Ok(player) = move_player(id.unwrap(), message.payload) {
                        println!("Moved player");
                        // sender
                        //     .send_message(&OwnedMessage::Text(
                        //         Message {
                        //             _type: MessageType::NewPlayer,
                        //             payload: Some(player),
                        //         }
                        //         .into(),
                        //     ))
                        //     .expect("Could not send coords");
                        sender
                            .send_message(&OwnedMessage::Text(
                                Message {
                                    _type: MessageType::PlayerCoords,
                                    payload: Some(PLAYERS.lock().unwrap().clone()),
                                }
                                .into(),
                            ))
                            .expect("Could not send coords");
                    }
                }
                _ => {}
            }
        }
    }
    return None;
}

fn launch_server() {
    let server = Server::bind("0.0.0.0:2794").expect("Could not bind server");

    for request in server.filter_map(Result::ok) {
        println!("LA BONNE SUCK IT");
        thread::spawn(|| {
            let client = request.accept().expect("Could not accept client");

            let ip = client.peer_addr().expect("Could not get client IP");

            info!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().expect("Could not split client");
            let mut id: Option<usize> = None;

            for message in receiver.incoming_messages() {
                let message = message.expect("Could not get incoming message");

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender
                            .send_message(&message)
                            .expect("Could not send close message");
                        info!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender
                            .send_message(&message)
                            .expect("Could not send pong message");
                    }
                    _ => {
                        if let Some(new_id) = handle_message(id, &message, &mut sender) {
                            id = Some(new_id);
                        }
                    }
                }
            }
        });
    }
}

fn main() {
    pretty_env_logger::init();

    launch_server();

    // println!(
    //     "{}",
    //     serde_json::to_string(&map).expect("Could not serialize")
    // );
}
