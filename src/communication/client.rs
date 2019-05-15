use serde::Serialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use websocket::sender::Writer;
use websocket::OwnedMessage;

use crate::communication::{OutgoingMessage, OutgoingMessageType};

lazy_static! {
    pub static ref CLIENTS: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(vec![]));
}

pub type Socket = Writer<TcpStream>;

pub struct Client {
    pub id: Uuid,
    pub sender: Writer<TcpStream>,
}

pub fn add_client(id: Uuid, sender: Socket) {
    CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .push(Client { id, sender });
}

pub fn remove_client(id: Uuid) {
    CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .retain(|c| c.id != id);
}

pub fn with_client_id(id: Uuid, cb: &Fn(&mut Writer<TcpStream>)) {
    if let Some(client) = CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .iter_mut()
        .filter(|c| c.id == id)
        .collect::<Vec<_>>()
        .first_mut()
    {
        cb(&mut client.sender);
    } else {
        error!("Client not found");
    }
}

pub fn send_all_clients<T>(message_type: OutgoingMessageType, payload: Option<T>)
where
    T: Serialize,
{
    let message: String = OutgoingMessage {
        _type: message_type,
        payload,
    }
    .into();

    for client in CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .iter_mut()
    {
        client
            .sender
            .send_message(&OwnedMessage::Text(message.clone()))
            .expect("Could not send position to player {}")
    }
}
