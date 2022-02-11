mod game;


fn main() {
    match game::Game::from_file("game.yaml") {
        Ok(mut game) => game.play(),
        Err(info) => println!("Error: {}", info)
    }
}
