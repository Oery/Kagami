use serde::{Deserialize, Serialize};

use crate::packets::json::ChatComponent;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub description: ChatComponent,
    pub players: Players,
    pub version: Version,
    pub favicon: String,
}
