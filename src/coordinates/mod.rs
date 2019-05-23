/// Represents 2D coordinates.
#[derive(Serialize, Deserialize, Clone)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Default for Coords {
    fn default() -> Self {
        Coords { x: 0.0, y: 0.0 }
    }
}
