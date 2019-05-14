use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;
use rand::Rng;
use std::collections::HashMap;

use crate::map::tiles::{Tile, TileType};

lazy_static! {
    pub static ref MAP: Map = Map::generate_map(100, 100);
}

#[derive(Debug, Serialize, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    content: HashMap<String, Vec<Tile>>,
}

impl Map {
    pub fn generate_map(width: usize, height: usize) -> Self {
        let mut map = Map {
            width,
            height,
            content: HashMap::new(),
        };

        map.generate_land();
        map.generate_objects();

        map
    }

    pub fn generate_land(&mut self) -> &Self {
        let seed = rand::thread_rng().gen();
        let noise_generator = OpenSimplex::new();
        let noise_generator = noise_generator.set_seed(seed);

        for y in 0..self.width {
            for x in 0..self.height {
                let depth: u32 =
                    ((noise_generator.get([x as f64 / 20.0, y as f64 / 20.0]) + 1.0) * 10.0) as u32;
                let key = format!("{};{}", x, y);
                let tile_type = match depth {
                    5...9 => TileType::Water,
                    9...10 => TileType::Sand,
                    10...15 => TileType::Grass,
                    _ => TileType::Grass,
                };
                let walkable = tile_type.is_walkable();

                self.content.insert(
                    key,
                    vec![Tile {
                        x,
                        y,
                        tile_type,
                        index: 0,
                        walkable,
                    }],
                );
            }
        }

        self
    }

    pub fn generate_objects(&mut self) -> &Self {
        let mut random = rand::thread_rng();

        for y in 0..(self.width as i32) {
            for x in 0..(self.height as i32) {
                let key = format!("{};{}", x, y);

                if let Some(tile) = self.content.get_mut(&key) {
                    let base_tile: &Tile =
                        tile.iter().filter(|t| t.index == 0).collect::<Vec<&Tile>>()[0];

                    if base_tile.tile_type == TileType::Grass && random.gen::<usize>() % 30 == 0 {
                        tile.push(Tile {
                            x: x as usize,
                            y: y as usize,
                            tile_type: match random.gen::<usize>() % 3 {
                                0 => TileType::Rock,
                                _ => TileType::Tree,
                            },
                            index: 0,
                            walkable: false,
                        });
                    }
                }
            }
        }

        // for (int yn = yc - spacing; yn <= yc + spacing; yn++) {
        //   for (int xn = xc - spacing; xn <= xc + spacing; xn++) {
        //     double e = value[yn][xn];
        //     if (e > max) { max = e; }
        //   }
        // }
        // if (value[yc][xc] == max) {
        //   // place tree at xc,yc
        // }

        self
    }
}
