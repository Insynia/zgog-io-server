pub mod player;

pub use player::Player;

use rand::Rng;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::coordinates::Coords;
use crate::map::MAP;

lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(vec![]));
}

pub fn add_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<Player, ()> {
    let mut random = rand::thread_rng();

    if let Some(payload) = payload {
        if let Some(name) = payload["name"].as_str() {
            let player = Player {
                id,
                name: name.to_owned(),
                x: (random.gen::<usize>() % MAP.width) as f64,
                y: (random.gen::<usize>() % MAP.width) as f64,
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
                player.x = coords.x;
                player.y = coords.y;

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
