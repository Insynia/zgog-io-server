#[derive(Debug, Clone, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum TileType {
    Water,
    Sand,
    Grass,
    Tree,
    Rock,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    #[serde(rename = "type")]
    pub tile_type: TileType,
    pub index: usize, //TODO: Needed ?
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
