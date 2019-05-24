use serde::Serialize;

/// A message type for incoming clients' messages.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncomingMessageType {
    /// Used to ask the map.
    /// Needs no payload.
    Map,
    /// Used to tell that a player just moved.
    /// Must be provided alongside with a [PlayerCoords](crate::player::PlayerCoords)
    /// struct as payload.
    PlayerCoords,
    /// Used to tell that a player just connected.
    /// Must be provided alongside with a [NewPlayerInfos](crate::player::NewPlayerInfos)
    /// struct as payload.
    NewPlayer,
}

/// A message type for outgoing server's messages.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutgoingMessageType {
    /// Used to send the map.
    /// Must be provided alongside with a [Map](crate::map::Map) struct as payload.
    Map,
    /// Used to send the hero's informations.
    /// Must be provided alongside with a [Player](crate::player::Player) struct as payload.
    Hero,
    /// Used to tell that a new player just connected.
    /// Must be provided alongside with a [Player](crate::player::Player) struct as payload.
    NewPlayer,
    /// Used to send all player's informations.
    /// Must be provided alongside with a vector of [Player](crate::player::Player) structs
    /// as payload.
    AllPlayers,
    /// Used to send a player's update.
    /// Must be provided alongside with a [Player](crate::player::Player) struct as payload.
    PlayerUpdated,
}

/// Message receveived from a client websocket. The type is used to identify
/// the request and the payload (optionnal) to handle it. If a request needs
/// a payload but the client does not provide it in its message, the request
/// will be ignored.
#[derive(Serialize, Deserialize)]
pub struct IncomingMessage<T> {
    #[serde(rename = "type")]
    pub _type: IncomingMessageType,
    pub payload: Option<T>,
}

/// Message send by the server to a client websocket. The type is used to identify
/// the request and the payload (optionnal) to handle it. The payload must implement
/// [Serialize](serde::Serialize) so it can be serialized to string and sent through
/// a message.
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
