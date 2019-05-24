#[derive(Serialize_repr, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum ObjectType {
    Tree,
    Rock,
}

/// Represents an object. It can either be a "map object" such as
/// tree/rock or other things like loot. These should not be walkable.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    #[serde(rename = "type")]
    pub _type: ObjectType,
    pub size: usize,
}
