#[derive(Serialize_repr, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum MapObjectType {
    Tree,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapObject {
    #[serde(rename = "type")]
    pub _type: MapObjectType,
    pub size: usize,
}
