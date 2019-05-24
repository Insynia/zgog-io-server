#[derive(Serialize_repr, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum ObjectType {
    Tree,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    #[serde(rename = "type")]
    pub _type: ObjectType,
    pub size: usize,
}
