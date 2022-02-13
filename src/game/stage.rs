use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::game::Game;

use super::path::Path;
use super::item::Item;
use super::trigger::Trigger;

#[derive(Serialize, Deserialize, Clone)]
pub struct Stage {
    pub description: String,
    pub paths: HashMap<String, Path>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub triggers: HashMap<String, Trigger>
}

impl Stage {

    pub fn get_path(&self, path_ident: &String) -> Option<&Path> {
        self.paths.get(path_ident)
    }
} // impl stage