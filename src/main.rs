use crate::framework::game::{Game, MyGame, GameHandler, GameTime, GameWindow};
use crate::shared::XnaResult;

mod shared;
mod framework;
mod csharp;

fn main() {
    let mut game = Game::new(Box::new(Game1::default()));

    let mut result = game.run();

    if result.is_err() {
       println!("{}", result.err().unwrap());
    }

    println!("Hello, world!");
}


#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct Game1 {
}

impl MyGame for Game1 {
    fn initialize(&mut self, game: GameHandler) -> XnaResult<()> {
        Ok(())
    }

    fn load_content(&mut self, game: GameHandler) -> XnaResult<()> {
        Ok(())
    }

    fn update(&mut self, game_time: GameTime, game: GameHandler) -> XnaResult<()> {
        Ok(())
    }

    fn draw(&mut self, game_time: GameTime, game: GameHandler) -> XnaResult<()> {
        Ok(())
    }
}
