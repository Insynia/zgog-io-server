use serde::Serialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use websocket::sender::Writer;
use websocket::OwnedMessage;

use crate::communication::{OutgoingMessage, OutgoingMessageType};

lazy_static! {
    /// Contains all the clients that are currently connected to the server.
    pub static ref CLIENTS: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(vec![]));
}

/// An alias to `Writer<TcpStream>`.
pub type Socket = Writer<TcpStream>;

/// Represents a client connected to the server.
pub struct Client {
    /// Unique id that identify the client.
    pub id: Uuid,
    /// Socket used to write messages to the client.
    pub sender: Writer<TcpStream>,
}

/// Add a client to the clients list.
pub fn add_client(id: Uuid, sender: Socket) {
    CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .push(Client { id, sender });
}

/// Remove a client from the clients list.
pub fn remove_client(id: Uuid) {
    CLIENTS
        .lock()
        .expect("Could not lock clients mutex")
        .retain(|c| c.id != id);
}

/// Allows to acess the client's writing socket in a closure. It allows
/// to send messages to a specific client from it's id.
///
///
/// Example:
///
/// ```
/// with_client_id(id, &|s: &mut Socket| {
///     s.send_message(&OwnedMessage::Text("Hello"))
/// });
/// ```
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

/// Sends a message to all the clients. The message is built from the provided parameters.
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
