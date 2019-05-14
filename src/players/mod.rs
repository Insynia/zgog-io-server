pub mod player;

pub use player::Player;

use std::sync::{Arc, Mutex};

use crate::map::MAP;

lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(vec![]));
}

pub fn add_player(payload: Option<serde_json::Value>) -> Result<Player, ()> {
    if let Some(payload) = payload {
        if let Some(name) = payload["name"].as_str() {
            let player = Player {
                id: match PLAYERS.lock().unwrap().last() {
                    Some(player) => player.id + 1,
                    _ => 0,
                },
                name: name.to_owned(),
                x: 0.0,
                y: 0.0,
            };

            PLAYERS
                .lock()
                .expect("Could not lock players mutex")
                .push(player.clone());
            return Ok(player);
        }
    }
    Err(())
}

#[derive(Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

pub fn move_player(id: usize, payload: Option<serde_json::Value>) -> Result<Player, ()> {
    if let Some(payload) = payload {
        if let Ok(coords) =
            serde_json::from_str::<Coords>(&serde_json::to_string(&payload).unwrap())
        {
            if let Some(ref mut player) = PLAYERS
                .lock()
                .unwrap()
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
