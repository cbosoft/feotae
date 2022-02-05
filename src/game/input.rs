use std::io::{self, Write};

#[derive(PartialEq, Debug)]
pub enum Input {
    Go(String),
    Look(String),
    // Take(String),
    Search,
    NoOp
}

fn unrecognised(input: &String) -> Input {
    println!("Unrecognised input: {}", input);
    Input::NoOp
}

fn parse_input_str(input: &str) -> Input {
    let tokens = input.split(' ').collect::<Vec<_>>();
    let n = tokens.len();
    match n {
        0 => Input::NoOp,
        1 => match tokens[0] {
            "search" => Input::Search,
            _ => unrecognised(&input.to_string())
        },
        2 => match tokens[0] {
            "go" => Input::Go(tokens[1].to_string()),
            "look" => Input::Look(tokens[1].to_string()),
            "examine" => Input::Look(tokens[1].to_string()),
            _ => unrecognised(&input.to_string())
        }
        _ => Input::NoOp,
    }
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

mod tests {

    use crate::game::input::{Input, parse_input_str};

    #[test]
    fn parse_test() {
        assert_eq!(parse_input_str(""), Input::NoOp);
        assert_eq!(parse_input_str("go north"), Input::Go("north".to_string()));
        assert_eq!(parse_input_str("look north"), Input::Look("north".to_string()));
        assert_eq!(parse_input_str("examine spade"), Input::Look("spade".to_string()));
        assert_eq!(parse_input_str("search"), Input::Search);
    }
}