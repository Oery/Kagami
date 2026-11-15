use serde::Deserialize;

// WARN: The profile id in the json response
// has format "11111111222233334444555555555555"
// which needs to be changed into format "11111111-2222-3333-4444-555555555555"
// before sending it back to the client.

#[derive(Deserialize, Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub name: String,
    pub properties: Vec<Property>,
}
