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
