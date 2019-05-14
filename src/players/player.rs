use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub x: f64,
    pub y: f64,
}
