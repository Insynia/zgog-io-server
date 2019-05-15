pub mod player;

pub use player::Player;

use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::coordinates::Coords;

lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(vec![]));
}

//TODO: Random coordinates at spawn
pub fn add_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<Player, ()> {
    if let Some(payload) = payload {
        if let Some(name) = payload["name"].as_str() {
            let player = Player {
                id,
                name: name.to_owned(),
                coords: Coords { x: 0.0, y: 0.0 },
                velocity: Coords { x: 0.0, y: 0.0 },
            };

            PLAYERS
                .lock()
                .expect("Could not lock players mutex")
                .push(player.clone());
            info!("New player \"{}\" with id \"{}\"", player.name, player.id);
            return Ok(player);
        }
    }
    Err(())
}

pub fn move_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<Player, ()> {
    if let Some(payload) = payload {
        if let Ok(coords) = serde_json::from_value::<Coords>(payload) {
            if let Some(ref mut player) = PLAYERS
                .lock()
                .expect("Could not lock players mutex")
                .iter_mut()
                .filter(|p| p.id == id)
                .collect::<Vec<_>>()
                .first_mut()
            {
                player.coords.x = coords.x;
                player.coords.y = coords.y;

                return Ok(player.clone());
            }
        }
    }
    Err(())
}

use std::net::TcpStream;
use websocket::result::WebSocketError;
use websocket::sender::Writer;
use websocket::OwnedMessage;

use crate::communication::{OutgoingMessage, OutgoingMessageType};

pub fn send_player(sender: &mut Writer<TcpStream>, player: Player) -> Result<(), WebSocketError> {
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::PlayerUpdated,
            payload: Some(player),
        }
        .into(),
    ))
}

pub fn send_new_player(
    sender: &mut Writer<TcpStream>,
    player: Player,
) -> Result<(), WebSocketError> {
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::NewPlayer,
            payload: Some(player),
        }
        .into(),
    ))
}

pub fn send_all_players(sender: &mut Writer<TcpStream>) -> Result<(), WebSocketError> {
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::AllPlayers,
            payload: Some(
                PLAYERS
                    .lock()
                    .expect("Could not lock players mutex")
                    .clone(),
            ),
        }
        .into(),
    ))
}
