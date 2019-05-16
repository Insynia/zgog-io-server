use uuid::Uuid;

use crate::coordinates::Coords;

/// Player struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
}

/// Player coords struct
#[derive(Serialize, Deserialize)]
pub struct PlayerCoords {
    pub position: Coords,
    pub orientation: Coords,
    pub velocity: Coords,
}

/// NewPlyerInfos struct
#[derive(Serialize, Deserialize)]
pub struct NewPlayerInfos {
    pub name: String,
}
