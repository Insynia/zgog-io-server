pub mod inventory;

use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::coordinates::Coords;
use crate::map::valid_spawn;
use crate::player::inventory::Inventory;

/// Player struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
    pub inventory: Inventory,
}

/// Player coords struct
#[derive(Serialize, Deserialize)]
pub struct PlayerCoords {
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
}

/// NewPlyerInfos struct
#[derive(Serialize, Deserialize)]
pub struct NewPlayerInfos {
    pub name: String,
}

lazy_static! {
    pub static ref PLAYERS: Arc<RwLock<Vec<Player>>> = Arc::new(RwLock::new(vec![]));
}

pub fn add_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<Player, String> {
    if let Some(payload) = payload {
        if let Ok(player) = serde_json::from_value::<NewPlayerInfos>(payload) {
            let player = Player {
                id,
                name: player.name.to_owned(),
                position: valid_spawn(),
                orientation: Coords::default(),
                velocity: Coords::default(),
                inventory: Inventory::default(),
            };

            PLAYERS
                .write()
                .expect("Could not lock players mutex")
                .push(player.clone());
            info!("New player \"{}\" with id \"{}\"", player.name, player.id);
            return Ok(player);
        } else {
            Err("Could not deserialize player infos for add_player".to_owned())
        }
    } else {
        Err("No payload provided for add_player".to_owned())
    }
}

pub fn remove_player(id: Uuid) {
    PLAYERS
        .write()
        .expect("Could not lock players mutex")
        .retain(|c| c.id != id);
    info!("Player with id \"{}\" removed", id);
}

pub fn move_player(id: Uuid, payload: Option<serde_json::Value>) -> Result<(), String> {
    if let Some(payload) = payload {
        if let Ok(coords) = serde_json::from_value::<PlayerCoords>(payload) {
            if let Some(ref mut player) = PLAYERS
                .write()
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

                Ok(())
            } else {
                Err("Player not found for move_player".to_owned())
            }
        } else {
            Err("Could not deserialize coords for move_player".to_owned())
        }
    } else {
        Err("No payload provided for move_player".to_owned())
    }
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
                    .read()
                    .expect("Could not lock players mutex")
                    .clone(),
            ),
        }
        .into(),
    ))
}
