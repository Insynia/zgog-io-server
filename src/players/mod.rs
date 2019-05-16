pub mod player;

pub use player::{NewPlayerInfos, Player, PlayerCoords};

use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::coordinates::Coords;
use crate::map::valid_spawn;

lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(vec![]));
}

pub fn add_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<Player, ()> {
    if let Some(payload) = payload {
        if let Ok(player) = serde_json::from_value::<NewPlayerInfos>(payload) {
            let player = Player {
                id,
                name: player.name.to_owned(),
                position: valid_spawn(),
                orientation: Coords { x: 0.0, y: 0.0 },
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
        if let Ok(coords) = serde_json::from_value::<PlayerCoords>(payload) {
            if let Some(ref mut player) = PLAYERS
                .lock()
                .expect("Could not lock players mutex")
                .iter_mut()
                .filter(|p| p.id == id)
                .collect::<Vec<_>>()
                .first_mut()
            {
                player.position.x = coords.position.x;
                player.position.y = coords.position.y;
                player.orientation.x = coords.orientation.x;
                player.orientation.y = coords.orientation.y;
                player.velocity.x = coords.velocity.x;
                player.velocity.y = coords.velocity.y;

                return Ok(player.clone());
            }
        } else {
            warn!("Could not deserialize coords for move_player");
        }
    }
    Err(())
}

use std::net::TcpStream;
use websocket::result::WebSocketError;
use websocket::sender::Writer;
use websocket::OwnedMessage;

use crate::communication::{OutgoingMessage, OutgoingMessageType};

pub fn send_hero(sender: &mut Writer<TcpStream>, player: Player) -> Result<(), WebSocketError> {
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::Hero,
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
