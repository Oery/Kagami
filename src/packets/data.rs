use serde::Deserialize;
use serde::Serialize;

// TODO: Add player sample
#[derive(Debug, Deserialize, Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub description: String,
    pub players: Players,
    pub version: Version,
    pub favicon: String,
}
