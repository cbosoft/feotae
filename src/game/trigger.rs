use serde::{Serialize, Deserialize};

fn default_trigger_action() -> String {
    "toggle".to_string()
}

fn default_trigger_visibility() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Trigger {
    pub flag: String,
    pub description: String,
    #[serde(default="default_trigger_visibility")]
    pub visible: bool,
    #[serde(default="default_trigger_action")]
    pub action: String
}