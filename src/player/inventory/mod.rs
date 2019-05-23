#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    stone: usize,
    wood: usize,
    gold: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            stone: 0,
            wood: 0,
            gold: 0,
        }
    }
}
