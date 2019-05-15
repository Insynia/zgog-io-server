use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncomingMessageType {
    Map,
    PlayerCoords,
    NewPlayer,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutgoingMessageType {
    Map,
    Hero,
    NewPlayer,
    AllPlayers,
    PlayerUpdated,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingMessage<T>
where
    T: Serialize,
{
    #[serde(rename = "type")]
    pub _type: IncomingMessageType,
    pub payload: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct OutgoingMessage<T>
where
    T: Serialize,
{
    #[serde(rename = "type")]
    pub _type: OutgoingMessageType,
    pub payload: Option<T>,
}

impl<T> Into<String> for OutgoingMessage<T>
where
    T: Serialize,
{
    fn into(self) -> String {
        serde_json::to_string(&self).expect("Could not serialize message")
    }
}
