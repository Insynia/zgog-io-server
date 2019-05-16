use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;
use rand::Rng;
use std::collections::HashMap;

use crate::coordinates::Coords;
use crate::map::tiles::{Tile, TileType};

lazy_static! {
    pub static ref MAP: Map = generate_map(30, 30);
}

/// A map. The `width` and `height` fields represent the size of the map
/// and the content is a [HashMap](HashMap) containing all the squares.
/// Each square of the map is accessible through the key `x;y`, `x` and
/// `y` being the coordinates of the tile. The value is then a vector of
/// [Tile](Tile) which represent each element present on the square.
#[derive(Debug, Serialize, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    content: HashMap<String, Vec<Tile>>,
}

/// Generate a map of a size provided in parameters.
pub fn generate_map(width: usize, height: usize) -> Map {
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
            let mut content = vec![];
            let tile_type = match depth {
                5...9 => TileType::Water,
                9...10 => TileType::Sand,
                10...15 => TileType::Grass,
                _ => TileType::Grass,
            };
            let walkable = tile_type.is_walkable();

            content.push(Tile {
                x,
                y,
                _type: tile_type,
                index: 0,
                walkable,
            });
            if walkable && random.gen::<usize>() % 30 == 0 {
                content.push(Tile {
                    x: x as usize,
                    y: y as usize,
                    _type: match random.gen::<usize>() % 3 {
                        0 => TileType::Rock,
                        _ => TileType::Tree,
                    },
                    index: 0,
                    walkable: false,
                });
            }
            map.content.insert(key, content);
        }
    }

    map
}

/// Either a tile is walkable or not. It loops through
/// all the subtiles of a Tile to check if one is not walkable.
/// If so, the whole tile won't be.
fn is_walkable(x: usize, y: usize) -> bool {
    let mut walkable = true;
    let tile = MAP
        .content
        .get(&format!("{};{}", x, y))
        .expect("Tile not found");

    for sub_tile in tile {
        if !sub_tile.walkable {
            walkable = false;
        }
    }

    walkable
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
    sender.send_message(&OwnedMessage::Text(
        OutgoingMessage {
            _type: OutgoingMessageType::Map,
            payload: Some(MAP.clone()),
        }
        .into(),
    ))
}
