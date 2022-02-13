use std::io::{self, Write};

use pcre::{Pcre, Match};

#[derive(PartialEq, Debug, Clone)]
pub enum Input {
    Go(String),
    Look(String),
    Take(String),
    Use(String),
    UseWith(String, String),
    Search,
    NoOp,

    Save(Option<String>),
    Load(Option<String>),
    Exit
}

pub enum InputSpec {
    Go, Look, Take, Use, Search, Save, Load, Exit
}

struct InputPattern {
    pattern: &'static str,
    input: InputSpec
}

impl InputPattern {
    pub const fn new(pattern: &'static str, input: InputSpec) -> InputPattern {
        InputPattern{
            pattern,
            input
        }
    }

    pub fn get_input(&self, input: &String) -> Result<Input, String> {
        let mut regex = Pcre::compile(self.pattern).unwrap();
        let matches = regex.matches(input).collect::<Vec<Match>>();
        if matches.len() == 1 {
            let m = matches.first().unwrap();

            Ok(match self.input {
                InputSpec::Go => Input::Go(m.group(1).to_string()),
                InputSpec::Look => Input::Look(m.group(1).to_string()),
                InputSpec::Take => Input::Take(m.group(1).to_string()),
                InputSpec::Use => if regex.capture_count() == 1 {
                    Input::Use(m.group(1).to_string())
                } else {
                    Input::UseWith(m.group(1).to_string(), m.group(2).to_string())
                },
                InputSpec::Search => Input::Search,
                InputSpec::Save => Input::Save(
                    if regex.capture_count() == 0 {
                        None
                    }
                    else {
                        Some(m.group(1).to_string())
                    }
                ),
                InputSpec::Load => Input::Load(
                    if regex.capture_count() == 0 {
                        None
                    }
                    else {
                        Some(m.group(1).to_string())
                    }
                ),
                InputSpec::Exit => Input::Exit
            })
        }
        else if matches.len() > 1 {
            Err("too many matches".to_string())
        }
        else {
            Err("no match".to_string())
        }
    }
}

static INPUT_PATTERNS: [InputPattern; 10] =
[
    InputPattern::new(r"(?:go|enter) (\w+)",InputSpec::Go),
    InputPattern::new(r"(?:look(?: at)?|examine) (\w+)",InputSpec::Look),
    InputPattern::new(r"search", InputSpec::Search),
    InputPattern::new(r"take (.*)", InputSpec::Take),
    InputPattern::new(r"use (\w+)", InputSpec::Use),
    //InputPattern::new("attack (\w+) (?:with (\w+))?", 2)

    // "menu" commands
    InputPattern::new(r"exit", InputSpec::Exit),
    InputPattern::new(r"save", InputSpec::Save),
    InputPattern::new(r"save (\w+)", InputSpec::Save),
    InputPattern::new(r"load", InputSpec::Load),
    InputPattern::new(r"load (\w+)?", InputSpec::Load),
];

fn parse_input_str(input: &str) -> Input {
    let sinput = input.to_string();
    let mut err_reason = String::new();
    for input_pattern in &INPUT_PATTERNS {
        match input_pattern.get_input(&sinput) {
            Ok(result) => return result,
            Err(error) => err_reason = error
        }
    }

    println!("Unrecognised input: \"{}\" ({})", input, err_reason);
    Input::NoOp
}

pub fn get_input() -> Input {
    print!("\n> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input){
        input.pop(); // remove training newline
        parse_input_str(&input)
    }
    else {
        Input::NoOp
    }
}

#[cfg(test)]
mod tests {

    use crate::game::input::{Input, parse_input_str};

    #[test]
    fn parse_test() {
        assert_eq!(parse_input_str(""), Input::NoOp);
        assert_eq!(parse_input_str("go north"), Input::Go("north".to_string()));
        assert_eq!(parse_input_str("look north"), Input::Look("north".to_string()));
        assert_eq!(parse_input_str("examine spade"), Input::Look("spade".to_string()));
        assert_eq!(parse_input_str("search"), Input::Search);
        assert_eq!(parse_input_str("take"), Input::NoOp);
        assert_eq!(parse_input_str("take all"), Input::Take("all".to_string()));
    }
}