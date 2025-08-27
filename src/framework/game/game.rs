use crate::framework::game::{Game, GameTime, GameWindow, RefGame, StepTimer};
use crate::null_pointer_exception;
use crate::shared::{Ptr, XnaResult, Exception};

impl Game {
    pub fn new() -> Self {
        Self {
            reference: Ptr::new(RefGame::default()),
        }
    }
    
    pub fn initialize(&mut self) -> XnaResult<()> {
        if self.reference.is_null() {
            self.reference = Ptr::new(RefGame::default());
        }
        
        let mut window = GameWindow::default();
        let result = window.create();

        if result.is_err() {
            println!("{}", result.err().unwrap());
        } else {
            self.start_game_loop()?
        }
        
        Ok(())
    }
    
    pub fn load_content() -> XnaResult<()> {
        unimplemented!()
    }
    
    pub fn update(&mut self, game_time: &GameTime) -> XnaResult<()> {
        unimplemented!()
    }

    pub fn draw(&mut self, game_time: &GameTime) -> XnaResult<()> {
        unimplemented!()
    }

    pub fn exit(&mut self) -> XnaResult<()> {
        self.reference
            .try_get_mut(null_pointer_exception!())?
            .game_window.
            close()
    }
}