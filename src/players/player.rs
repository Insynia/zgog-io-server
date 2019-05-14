#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub x: f64,
    pub y: f64,
}
