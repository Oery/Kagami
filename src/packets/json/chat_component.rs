use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatComponent {
    Node(ChatNode),
    TranslateNode(TranslateNode),
    Text(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatNode {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub extra: Vec<ChatComponent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranslateNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub translate: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub with: Vec<ChatComponent>,
}
