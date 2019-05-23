#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MapObjectType {
    Tree,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapObject {
    #[serde(rename = "type")]
    pub _type: MapObjectType,
}
