use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use serde::Deserialize;

use super::input::{Input, get_input};
use super::stage::Stage;
use super::parse::get_contents;
use super::item::Item;
use super::path::Path;



#[derive(Deserialize)]
pub struct Game {
    name: String,
    description: String,
    inventory: HashMap<String, Item>,
    player_inventory: Vec<String>,
    stages: HashMap<String, Stage>,
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
    pub fn from_toml(filename: &str) -> Result<Game, String> {
        let contents_str = get_contents(filename)?;
        match toml::from_str(&contents_str) {
            Ok(game) => Ok(game),
            Err(info) => Err(format!("could not parse toml: {:?}", info))
        }
    }

    fn current_stage(&self) -> &Stage {
        &self.stages[&self.current_stage]
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
            println!("{}", path.description);
        }
        else {
            println!("You look around, but see no \"{}\".", thing_name);
        }
    }

    fn search(&self) {
        let stage = self.current_stage();
        stage.search();
    }

    fn process_input(&mut self) {
        match get_input() {
            Input::Go(p) => self.go_dir(p),
            Input::Look(p) => self.look(p),
            Input::Search => self.search(),
            Input::NoOp => ()
        }
    }

    fn is_path_hidden(&self, path: &Path) -> bool {
        if let Some(ref cloaking_item_name) = path.hidden_by {
            if !self.player_inventory.contains(&cloaking_item_name) {
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

mod tests {

    use crate::game::game::Game;

    const WORKING_GAME_SOURCE: &str = r#"
name = "the great test"
description = "test game for testing"
current_stage = "first"
player_inventory = []

[inventory]
[inventory.1]
name = "blue key"
description = "a blue key"

[inventory.2]
name = "red key"
description = "a red key"

[inventory._first_east_hide]
name = "na"
description = "na"
hidden = true

[stages]
[stages.first]
description = "A test stage"

[stages.first.paths]
[stages.first.paths.north]
description = "a path to the north"
destination = "first"

[stages.first.paths.east]
description = "a path to the east"
destination = "first"
hidden_by = "_first_east_hide"
"#;

    #[test]
    fn parse_test() {
        println!(">>{}<<", WORKING_GAME_SOURCE);
        match toml::from_str::<Game>(WORKING_GAME_SOURCE) {
            Ok(_) => (),
            Err(s) => { println!("{}", s); panic!() }
        }
    }
}