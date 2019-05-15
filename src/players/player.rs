use uuid::Uuid;

use crate::coordinates::Coords;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerCoords {
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
}
