pub mod player;

pub use player::Player;

use std::sync::{Arc, Mutex};

use crate::map::MAP;

lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(vec![]));
}

pub fn add_player(payload: serde_json::Value) -> Result<Player, ()> {
    if let Some(name) = payload["name"].as_str() {
        let player = Player {
            name: name.to_owned(),
            x: MAP.width,
            y: MAP.height,
        };

        PLAYERS.push(player.clone());
        return Ok(player);
    }
    Err(())
}
