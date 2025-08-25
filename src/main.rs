use crate::framework::game::{Game, GameWindow};

mod shared;
mod framework;

fn main() {
    let mut game = Game::new();
    let mut result = game.initialize();

    if result.is_err() {
       println!("{}", result.err().unwrap());
    }

    println!("Hello, world!");
}
