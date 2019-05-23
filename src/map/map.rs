use log::debug;
use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;
use rand::Rng;
use std::collections::HashMap;

use crate::coordinates::Coords;
use crate::map::map_object::{MapObject, MapObjectType};
use crate::map::tiles::{Tile, TileType};

lazy_static! {
    pub static ref MAP: Map = generate_map(30, 30);
}

/// Represents the spacing between map objects. Around one object
/// every `MAP_OBJECTS_SPACING` tile (random distribution).
static MAP_OBJECTS_SPACING: usize = 30;

/// A map. The `width` and `height` fields represent the size of the map
/// and the content is a [HashMap](HashMap) containing all the squares.
/// Each square of the map is accessible through the key `x;y`, `x` and
/// `y` being the coordinates of the tile. The value is then a vector of
/// [Tile](Tile) which represent each element present on the square.
#[derive(Debug, Serialize, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    content: HashMap<String, Tile>,
}

/// Generate a map of a size provided in parameters.
pub fn generate_map(width: usize, height: usize) -> Map {
    debug!("Generating map of size: {}", height);
    let mut map = Map {
        width,
        height,
        content: HashMap::new(),
    };
    let mut random = rand::thread_rng();
    let seed = random.gen();
    let noise_generator = OpenSimplex::new();
    let noise_generator = noise_generator.set_seed(seed);

    for y in 0..map.width {
        for x in 0..map.height {
            let depth: u32 =
                ((noise_generator.get([x as f64 / 20.0, y as f64 / 20.0]) + 1.0) * 10.0) as u32;
            let key = format!("{};{}", x, y);
            let tile_type = match depth {
                5...9 => TileType::Water,
                9...10 => TileType::Sand,
                10...15 => TileType::Grass,
                _ => TileType::Grass,
            };
            let walkable = tile_type.walkable();
            let mut tile = Tile {
                x,
                y,
                _type: tile_type,
                content: vec![],
            };

            if walkable && random.gen_range(0, MAP_OBJECTS_SPACING) == 0 {
                tile.content.push(MapObject {
                    _type: match random.gen_range(1, 3 + 1) {
                        0 => MapObjectType::Rock,
                        _ => MapObjectType::Tree,
                    },
                    size: random.gen_range(1, 5 + 1),
                });
            }
            map.content.insert(key, tile);
        }
    }

    map
}

/// Either a tile is walkable or not. It loops through
/// all the subtiles of a Tile to check if one is not walkable.
/// If so, the whole tile won't be.
fn is_walkable(x: usize, y: usize) -> bool {
    let tile = MAP
        .content
        .get(&format!("{};{}", x, y))
        .expect("Tile not found");

    tile.walkable()
}

/// Returns valid coordinates to spawn a player (walkable tile).
pub fn valid_spawn() -> Coords {
    let mut random = rand::thread_rng();
    let (mut x, mut y) = (
        random.gen::<usize>() % MAP.width,
        random.gen::<usize>() % MAP.height,
    );

    while !is_walkable(x, y) {
        x = random.gen::<usize>() % MAP.width;
        y = random.gen::<usize>() % MAP.height;
    }

    debug!("Found a valid spawn at: {}/{}", x, y);

    Coords {
        x: x as f64,
        y: y as f64,
    }
}

use std::net::TcpStream;
use websocket::result::WebSocketError;
use websocket::sender::Writer;
use websocket::OwnedMessage;

use crate::communication::{OutgoingMessage, OutgoingMessageType};

/// Sends the map to a client.
pub fn send_map(sender: &mut Writer<TcpStream>) -> Result<(), WebSocketError> {
    debug!("Sending map...");
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::Map,
            payload: Some(MAP.clone()),
        }
        .into(),
    ))
}
