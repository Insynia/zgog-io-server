#[derive(Serialize_repr, Deserialize, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum VisualType {
    Water,
    Sand,
    Grass,
}

/// Represents a map visual. It can either be a base tile such as
/// grass/water and or decorations such as flowers.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Visual {
    #[serde(rename = "type")]
    pub _type: VisualType,
    pub size: usize,
}
