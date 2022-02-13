use std::collections::{HashMap, HashSet};
use std::env::var;
use std::fs;
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::process::exit;

use serde::{Serialize, Deserialize};

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

fn default_savename() -> String {
    "default".to_string()
}


#[derive(Serialize, Deserialize, Clone)]
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
    current_stage: String,
    #[serde(skip_serializing_if = "String::is_empty", default="default_savename")]
    savename: String
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

    pub fn play(&mut self) {
        println!("---");
        println!("{}", self.name);
        println!("{}", self.description);
        println!("---");
        loop {
            self.display(self.current_stage());
            self.process_input();
        }
    }

    fn move_to_stage(&mut self, stage_name: String) {
        self.current_stage = stage_name;
        self.run_trigger(&"on enter".to_string());
    }

    fn go_dir(&mut self, path_name: String) {
        match self.current_stage().get_path(&path_name) {
            Some(path) => {
                match self.is_path_traversable(path) {
                    Ok(destination) => self.move_to_stage(destination),
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
            println!("\n{}", path.detailed_description);
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
        let mut items = stage.items.clone();
        items.retain(|i| !self.player_inventory.contains(i));
        match Item::items_to_text(&items) {
            Some(txt) => println!("\nYou found {}", txt),
            None => println!("\nYou search the area, but find nothing.")
        }
    }

    fn take(&mut self, thing: String) {
        let stage = &self[&self.current_stage];
        if stage.items.contains(&thing) {
            if self.player_inventory.contains(&thing) {
                let a = get_article(&thing);
                println!("\nYou look around, but there's no {thing} to take. You panic, you know you saw {a} {thing} here a minute ago! Your panic subsides as you remember: you already took it.");
            }
            else {
                println!("\nYou take the {thing}.");
                self.player_inventory.push(thing);
            }
        }
        else {
            println!("\nYou look around, but there's no {thing} to take.");
        }
    }

    fn use_object(&mut self, object: String) {
        println!("use object \"{}\"", object);
        self.run_trigger(&format!("use {object}"));
    }

    fn use_item_with_object(&mut self, item: String, object: String) {
        println!("use item \"{}\" with object \"{}\"", item, object);
        if self.player_inventory.contains(&item) {
            self.run_trigger(&format!("use {item} on {object}"));
        }
        else {
            println!("\nYou pat your pockets, but realise you don't have {} {} to use.",  get_article(&item), &item)
        }
    }

    fn run_trigger(&mut self, trigger_name: &String) {
        let stage = self.stages[&(self.current_stage)].clone();
        if let Some(trigger) = stage.triggers.get(trigger_name) {
            if trigger.action == "toggle" {
                self.toggle_flag(&trigger.flag);
            }
            else {
                self.set_flag(&trigger.flag);
            }
        }
        else {
            println!("\nOh no, you can't do that.")
        }
    }

    fn toggle_flag(&mut self, flag_name: &String) {
        if self.is_flag_set(flag_name) {
            self.unset_flag(flag_name);
        }
        else {
            self.set_flag(flag_name);
        }
    }

    fn is_flag_set(&self, flag_name: &String) -> bool {
        self.flags.contains(flag_name)
    }

    fn set_flag(&mut self, flag_name: &String) {
        self.flags.insert((*flag_name).clone());
    }

    fn unset_flag(&mut self, flag_name: &String) {
        self.flags.remove(flag_name);
    }

    fn process_input(&mut self) {
        match get_input() {
            Input::Go(p) => self.go_dir(p),
            Input::Look(p) => self.look(p),
            Input::Search => self.search(),
            Input::Take(p) => self.take(p),
            Input::Use(object) => self.use_object(object),
            Input::UseWith(item, object) => self.use_item_with_object(item, object),
            Input::NoOp => (),

            // "menu" commands
            Input::Exit => exit(0),
            Input::Save(maybe_savename) => {
                self.update_savename(maybe_savename);
                self.save()
            },
            Input::Load(maybe_savename) => {
                self.update_savename(maybe_savename);
                self.load()
            },
        }
    }

    fn update_savename(&mut self, maybe_savename: Option<String>) {
        if let Some(savename) = maybe_savename {
            self.savename = savename;
        }
    }

    fn get_save_dir() -> String {
        let home = var("HOME").unwrap();
        format!("{home}/.feotae_saves")
    }

    fn get_save_dir_make_if_not_exists() -> String {
        let save_dir = Game::get_save_dir();
        if !std::path::Path::exists((&save_dir).as_ref()) {
            fs::create_dir(&save_dir);
        }
        save_dir
    }

    fn save(&self) {
        let save_dir = Game::get_save_dir_make_if_not_exists();
        self.save_to_file(format!("{}/{}.yaml", save_dir, self.savename));
    }

    fn save_to_file(&self, filepath: String) {
        let serd = serde_yaml::to_string(self).unwrap();
        let mut opener = fs::OpenOptions::new();
        opener.create(true);
        opener.write(true);
        opener.truncate(true);
        match opener.open(filepath) {
            Ok(mut f) => {
                f.write_fmt(format_args!("{}", serd));
                println!("\nGame saved successfully!")
            },
            Err(e) => println!("\nError saving game: {}", e.to_string())
        }
    }

    fn load(&mut self) {
        let save_dir = Game::get_save_dir();
        if !std::path::Path::exists((&save_dir).as_ref()) {
            println!("\nNo save games exist yet! Run \"save\" to save first.")
        }
        else {
            let load_file_name = format!("{}/{}.yaml", save_dir, self.savename);
            if !std::path::Path::exists((&load_file_name).as_ref()) {
                println!("\nSave game does not exist yet! Run \"save\" to save first.")
            }
            else {
                self.load_from_file(load_file_name)
            }
        }
    }

    fn load_from_file(&mut self, filepath: String) {
        match get_contents(&filepath) {
            Ok(contents) => {
                let g: Game = serde_yaml::from_str(&contents).unwrap();
                *self = g.clone();
            },
            Err(err_string) => println!("\nError loading game: {}", err_string)
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

    pub fn display(&self, stage: &Stage) {
        print!("\n{}", stage.description);
        for (_pathname,  path) in &stage.paths {
            if !self.is_path_hidden(path) {
                print!(" {}", path.description);
            }
        }
        for (_trigger_name,  trigger) in &stage.triggers {
            if trigger.visible {
                print!(" {}", trigger.description);
            }
        }
        println!();

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
      action:
        description: foo
        flag: "foo"
        visible: false

"#;

    #[test]
    fn parse_test() {
        match serde_yaml::from_str::<Game>(WORKING_GAME_SOURCE) {
            Ok(_) => (),
            Err(s) => { println!("{}", s); panic!() }
        }
    }
}