use uuid::Uuid;

use crate::coordinates::Coords;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub coords: Coords,
    pub velocity: Coords,
}
