use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};

use serde::Deserialize;

use super::{
    input::{
        Input,
        get_input
    },
    stage::Stage,
    parse::get_contents,
    item::{Item, get_article},
    path::Path,
    trigger::Trigger
};


#[derive(Deserialize)]
pub struct Game {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    inventory: HashMap<String, Item>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    player_inventory: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    stages: HashMap<String, Stage>,
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    flags: HashSet<String>,
    current_stage: String
}

impl Index<&String> for Game {
    type Output = Stage;

    fn index(&self, index: &String) -> &Self::Output {
        match self.stages.get(index) {
            Some(stage) => stage,
            None => panic!("attempted to access an undefined stage")
        }
    }
}

impl IndexMut<&String> for Game {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        match self.stages.get_mut(index) {
            Some(stage) => stage,
            None => panic!("attempted to access an undefined stage")
        }
    }
}

enum TraversalErrorKind {
    Locked,
    Hidden
}

impl Game {
    pub fn from_file(filename: &str) -> Result<Game, String> {
        let contents_str = get_contents(filename)?;
        match serde_yaml::from_str(&contents_str) {
            Ok(game) => Ok(game),
            Err(info) => Err(format!("could not parse file: {:?}", info))
        }
    }

    fn current_stage(&self) -> &Stage {
        &self[&self.current_stage]
    }

    fn current_stage_mut(&mut self) -> &mut Stage {
        let stage = self.current_stage.clone();
        &mut self[&stage]
    }

    pub fn play(&mut self) {
        println!("---");
        println!("{}", self.name);
        println!("{}", self.description);
        println!("---");
        loop {
            self.current_stage().display();
            self.process_input();
        }
    }

    fn go_dir(&mut self, path_name: String) {
        match self.current_stage().get_path(&path_name) {
            Some(path) => {
                match self.is_path_traversable(path) {
                    Ok(current_stage) => self.current_stage = current_stage,
                    Err(kind) => match kind {
                        TraversalErrorKind::Hidden => match path.hidden_text {
                            None => println!("Locked!"),
                            Some(ref s) => println!("{}", s)
                        },
                        TraversalErrorKind::Locked => match path.locked_text {
                            None => println!("Locked!"),
                            Some(ref s) => println!("{}", s)
                        }
                    }
                }
            },
            None => println!("Didn't understand \"{}\"", path_name)
        }
    }

    fn look(&self, thing_name: String) {
        let stage = self.current_stage();
        if let Some(path) = stage.get_path(&thing_name) {
            println!("\n{}", path.description);
        }
        else if let Some(item) = self.inventory.get(&thing_name) {
            println!("\n{}", item.description);
        }
        else {
            println!("\nYou look around, but see no \"{}\".", thing_name);
        }
    }

    fn search(&self) {
        let stage = self.current_stage();
        stage.search();
    }

    fn take(&mut self, thing_name: String) {
        let stage = self.current_stage_mut();
        if thing_name == "all".to_string() {
            if let Some(items) = stage.take_all() {
                self.player_inventory.extend(items);
            }
            else {
                println!("You look around, but there's nothing to take.");
            }
        } else {
            if let Some(thing) = stage.take(thing_name) {
                self.player_inventory.push(thing);
            }
            else {
                println!("You look around, but there's nothing to take.");
            }
        }
    }

    fn use_object(&mut self, object: String) {
        println!("use object \"{}\"", object);
        self.run_trigger("use ".to_string() + &object);
    }

    fn use_item_with_object(&mut self, item: String, object: String) {
        println!("use item \"{}\" with object \"{}\"", item, object);
        if self.player_inventory.contains(&item) {
            self.run_trigger("use ".to_string() + &item + " on " + &object);
        }
        else {
            println!("\nYou pat your pockets, but realise you don't have {} {} to use.",  get_article(&item), &item)
        }
    }

    fn run_trigger(&mut self, trigger: String) {
        if let Some(flag) = self.current_stage().triggers.get(&trigger) {
            self.flags.insert((*flag).clone());
        }
        else {
            println!("\nOh no, you can't do that.")
        }
    }

    fn process_input(&mut self) {
        match get_input() {
            Input::Go(p) => self.go_dir(p),
            Input::Look(p) => self.look(p),
            Input::Search => self.search(),
            Input::Take(p) => self.take(p),
            Input::Use(object) => self.use_object(object),
            Input::UseWith(item, object) => self.use_item_with_object(item, object),
            Input::NoOp => ()
        }
    }

    fn is_path_hidden(&self, path: &Path) -> bool {
        if let Some(ref flag) = path.hidden_unless {
            if !self.flags.contains(flag) {
                return true;
            }
        }
        return false;
    }

    fn is_path_locked(&self, path: &Path) -> bool {
        if let Some(ref locking_item_name) = path.locked_by {
            if !self.player_inventory.contains(&locking_item_name) {
                return true;
            }
        }
        return false;
    }

    fn is_path_traversable(&self, path: &Path) -> Result<String, TraversalErrorKind> {
        if self.is_path_hidden(path) {
            Err(TraversalErrorKind::Hidden)
        }
        else if self.is_path_locked(path) {
            Err(TraversalErrorKind::Locked)
        }
        else {
            Ok(path.destination.clone())
        }
    }


} // impl Game


#[cfg(test)]
mod tests {

    use crate::game::game::Game;

    const WORKING_GAME_SOURCE: &str = r#"
name: "the great test"
description: "test game for testing"
current_stage: "first"
inventory:
  blue key:
    name: blue key
    description: a blue key
stages:
  first:
    description: "A test stage"
    paths:
      north:
        description: "north path"
        destination: "first"
      east:
        description: "a path to the east"
        destination: "first"
        hidden_unless: "stage:first lever pulled"
    triggers:
      action: "foo"

"#;

    #[test]
    fn parse_test() {
        match serde_yaml::from_str::<Game>(WORKING_GAME_SOURCE) {
            Ok(_) => (),
            Err(s) => { println!("{}", s); panic!() }
        }
    }
}