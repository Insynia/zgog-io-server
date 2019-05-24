#[derive(Serialize_repr, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum VisualType {
    Water,
    Sand,
    Grass,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Visual {
    #[serde(rename = "type")]
    pub _type: VisualType,
    pub size: usize,
}
