use crate::framework::game::GameHandler;
use crate::null_pointer_exception;
use crate::shared::XnaResult;

impl GameHandler {
    pub fn exit(&mut self) -> XnaResult<()> {
        let mut game = self.game
            .try_get_mut(null_pointer_exception!())?;

        game.game_window.close()?;
        game.is_running = false;

        Ok(())
    }
}