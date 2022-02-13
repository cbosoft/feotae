use std::collections::HashMap;

use serde::Deserialize;
use crate::game::Game;

use super::path::Path;
use super::item::Item;

#[derive(Deserialize)]
pub struct Stage {
    pub description: String,
    pub paths: HashMap<String, Path>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub triggers: HashMap<String, Trigger>
}

impl Stage {

    pub fn search(&self) {
        match Item::items_to_text(&self.items) {
            Some(txt) => println!("\nYou found {}", txt),
            None => println!("\nYou search the area, but find nothing.")
        }
    }

    pub fn take_all(&mut self) -> Option<Vec<String>> {
        if self.items.len() > 0 {
            let items_cpy = self.items.clone();
            self.items.clear();
            Some(items_cpy)
        }
        else {
            None
        }
    }

    pub fn take(&mut self, item_name: String) -> Option<String> {
        if self.items.contains(&item_name) {
            self.items.retain(|x| x != &item_name);
            Some(item_name)
        }
        else {
            None
        }
    }

    pub fn get_path(&self, path_ident: &String) -> Option<&Path> {
        self.paths.get(path_ident)
    }
} // impl stage