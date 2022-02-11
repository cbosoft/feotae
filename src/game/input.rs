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
    NoOp
}

pub enum InputSpec {
    Go, Look, Take, Use, Search
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
                InputSpec::Use => match regex.capture_count() {
                    1 => Input::Use(m.group(1).to_string()),
                    _ => Input::UseWith(m.group(1).to_string(), m.group(2).to_string())
                }
                InputSpec::Search => Input::Search
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

static INPUT_PATTERNS: [InputPattern; 5] =
[
    InputPattern::new(r"go (\w+)",InputSpec::Go),
    InputPattern::new(r"(?:look(?: at)?|examine) (\w+)",InputSpec::Look),
    InputPattern::new(r"search", InputSpec::Search),
    InputPattern::new(r"take (\w+)", InputSpec::Take),
    InputPattern::new(r"use (\w+) (?:(?:with|on) (\w+))?", InputSpec::Use),
    //InputPattern::new("attack (\w+) (?:with (\w+))?", 2)
];

fn parse_input_str(input: &str) -> Input {
    let sinput = input.to_string();
    for input_pattern in &INPUT_PATTERNS {
        if let Ok(result) = input_pattern.get_input(&sinput) {
            return result;
        }
    }

    println!("Unrecognised input: \"{}\"", input);
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