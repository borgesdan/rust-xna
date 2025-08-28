use crate::framework::game::{Game, GameWindow};

mod shared;
mod framework;
mod csharp;

fn main() {
    let mut game = Game::new();
    let mut result = game.run();

    if result.is_err() {
       println!("{}", result.err().unwrap());
    }

    println!("Hello, world!");
}
