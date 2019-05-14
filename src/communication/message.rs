use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Map,
    PlayerCoords,
    NewPlayer,
}

#[derive(Serialize, Deserialize)]
pub struct Message<T>
where
    T: Serialize,
{
    #[serde(rename = "type")]
    pub _type: MessageType,
    pub payload: Option<T>,
}

impl<T> Into<String> for Message<T>
where
    T: Serialize,
{
    fn into(self) -> String {
        serde_json::to_string(&self).expect("Could not serialize message")
    }
}
