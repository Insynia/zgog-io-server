#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
}
