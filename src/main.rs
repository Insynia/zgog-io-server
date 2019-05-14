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
use crate::map::{Map, MAP};
use crate::players::add_player;

fn handle_message(message: &OwnedMessage, sender: &mut Writer<TcpStream>) {
    if let OwnedMessage::Text(str_message) = message {
        if let Ok(message) = serde_json::from_str::<Message<serde_json::Value>>(str_message) {
            match message._type {
                MessageType::Map => {
                    sender
                        .send_message(&OwnedMessage::Text(
                            Message {
                                _type: MessageType::Map,
                                payload: MAP.clone(),
                            }
                            .into(),
                        ))
                        .unwrap();
                    sender
                .send_message(&OwnedMessage::Text(
                    Message {
                        _type: MessageType::PlayerCoords,
                        payload: serde_json::from_str::<serde_json::Value>("[{ \"name\":\"Tibou\", \"x\": 5, \"y\": 7 }, { \"name\":\"terrylaput\", \"x\": 0, \"y\": 0 }]").expect("serde"),
                    }
                    .into(),
                ))
                .unwrap();
                }
                MessageType::NewPlayer => {
                    if let Ok(player) = add_player(message.payload) {

                    } else {

                    }
                }
                _ => {}
            }
        }
    }
}

fn launch_server() {
    let server = Server::bind("127.0.0.1:2794").expect("Could not bind server");

    for request in server.filter_map(Result::ok) {
        println!("LA BONNE SUCK IT");
        thread::spawn(|| {
            let client = request.accept().unwrap();

            let ip = client.peer_addr().unwrap();

            info!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        info!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    _ => handle_message(&message, &mut sender),
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
