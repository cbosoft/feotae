use serde::Deserialize;

#[derive(Deserialize)]
pub struct Path {
    pub description: String, // descriptive text shown to the player
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub detailed_description: String, // more descriptive text shown when looked closely at
    pub destination: String, // name of stage
    pub locked_by: Option<String>, // item that is required to unlock the path
    pub locked_text: Option<String>,
    pub hidden_unless: Option<String>,  // hidden from player unless flag is set
    pub hidden_text: Option<String>
}