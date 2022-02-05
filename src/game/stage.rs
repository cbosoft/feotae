use std::collections::HashMap;

use serde::Deserialize;

use super::path::Path;

#[derive(Deserialize)]
pub struct Stage {
    description: String,
    paths: HashMap<String, Path>,
    items: Option<Vec<String>>
}

fn get_article(word: &String) -> String {
    let a = "a".to_string();
    let an = "an".to_string();
    match &word[0..1] {
        "a" => an,
        "e" => an,
        "i" => an,
        "o" => an,
        "u" => an,
        _ => a
    }

}

impl Stage {

    pub fn display(&self) {
        println!("\n{}", self.description);
    }

    pub fn search(&self) {
        if let Some(ref items) = self.items {
            let n = items.len();
            if n == 1 {
                let ref item = items[0];
                println!("\nYou found {} {}.", get_article(&item), item);
            }
            else if n == 2 {
                let ref item1 = items[0];
                let ref item2 = items[1];
                println!("\nYou found {} {} and {} {}.", get_article(&item1), item1, get_article(&item2), item2);
            }
            else if n > 2 {
                println!("\nYou found:");
                for item in items {
                    println!(" - {} {}", get_article(&item), item);
                }
            }
            else {
                println!("\nYou search the area, but find nothing.");
            }
        }
        else {
            println!("\nYou search the area, but find nothing.");
        }
    }

    pub fn take(&self, item_name: String) {

    }

    pub fn get_path(&self, path_ident: &String) -> Option<&Path> {
        self.paths.get(path_ident)
    } 
} // impl stage

mod tests {

    use crate::game::stage::*;

    #[test]
    fn test_get_article() {
        let a = "a".to_string();
        let an = "an".to_string();
        assert_eq!(get_article(&"foo".to_string()), a);
        assert_eq!(get_article(&"abacus".to_string()), an);
        assert_eq!(get_article(&"yellow".to_string()), a);
    }
}