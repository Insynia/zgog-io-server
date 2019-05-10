#[macro_use]
extern crate log;

use std::net::TcpStream;
use std::thread;
use websocket::sender::Writer;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn handle_message(message: &OwnedMessage, sender: &mut Writer<TcpStream>) {
    if let OwnedMessage::Text(str_message) = message {
        if str_message == "coucou" {
            sender
                .send_message(&OwnedMessage::Text("salut".to_owned()))
                .unwrap();
        } else {
            sender
                .send_message(&OwnedMessage::Text(str_message.to_owned()))
                .unwrap();
        }
    }
}

fn main() {
    pretty_env_logger::init();

    let server = Server::bind("127.0.0.1:2794").expect("Could not bind server");

    for request in server.filter_map(Result::ok) {
        println!("LA BONNE SUCK IT");
        thread::spawn(|| {
            let mut client = request.accept().unwrap();

            let ip = client.peer_addr().unwrap();

            info!("Connection from {}", ip);

            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();

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
