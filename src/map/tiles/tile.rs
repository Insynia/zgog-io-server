/// Identify the type of a [Tile](Tile).
#[derive(Debug, Clone, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum TileType {
    Water,
    Sand,
    Grass,
    Tree,
    Rock,
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
    /// Used to set the z-order of the tiles with same coordinates.
    /// A tile with the index 0 is below a tile with the index 1.
    pub index: usize,
    /// Either you can walk on the tile or not.
    pub walkable: bool,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        if self == &TileType::Water {
            return false;
        }
        true
    }
}
