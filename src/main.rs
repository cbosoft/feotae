mod game;


fn main() {
    match game::Game::from_toml("game.toml") {
        Ok(mut game) => game.play(),
        Err(info) => println!("Error: {}", info)
    }
}
