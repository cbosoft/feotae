use std::collections::HashMap;

use serde::Deserialize;

use super::path::Path;
use super::item::Item;

#[derive(Deserialize)]
pub struct Stage {
    description: String,
    paths: HashMap<String, Path>,
    items: Option<Vec<String>>
}

impl Stage {

    pub fn display(&self) {
        println!("\n{}", self.description);
    }

    pub fn search(&self) {
        if let Some(ref items) = self.items {
            match Item::items_to_text(items) {
                Some(txt) => println!("\nYou found {}", txt),
                None => println!("\nYou search the area, but find nothing.")
            }
        }
        else {
            println!("\nYou search the area, but find nothing.");
        }
    }

    pub fn take_all(&mut self) -> Option<Vec<String>> {
        if let Some(items) = &mut self.items {
            let items_cpy = items.clone();
            items.clear();
            Some(items_cpy)
        }
        else {
            None
        }
    }

    pub fn take(&mut self, item_name: String) -> Option<String> {
        if let Some(items) = &mut self.items {
            if items.contains(&item_name) {
                items.retain(|x| x != &item_name);
                Some(item_name)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn get_path(&self, path_ident: &String) -> Option<&Path> {
        self.paths.get(path_ident)
    } 
} // impl stage