use crate::map::map_object::MapObject;

/// Identify the type of a [Tile](Tile).
#[derive(Debug, Clone, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum TileType {
    Water,
    Sand,
    Grass,
}

/// Represent a game tile.
#[derive(Debug, Clone, Serialize)]
pub struct Tile {
    /// The x position of the tile on the map.
    pub x: usize,
    /// The y position of the tile on the map.
    pub y: usize,
    #[serde(rename = "type")]
    /// The tile type.
    pub _type: TileType,
    pub objects: Vec<MapObject>,
    pub visuals: Vec<()>,
}

impl Tile {
    pub fn walkable(&self) -> bool {
        self._type.walkable()
    }
}

impl TileType {
    pub fn walkable(&self) -> bool {
        if self == &TileType::Water {
            return false;
        }
        true
    }
}
