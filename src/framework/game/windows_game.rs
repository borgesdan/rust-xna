use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, IsWindow, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::framework::game::{Game, GameTime, GameWindow, StepTimer};
use crate::{exception, null_pointer_exception};
use crate::csharp::TimeSpan;
use crate::shared::XnaResult;
use crate::shared::Exception;

impl Game {

    pub fn start_game_loop(&mut self) -> XnaResult<()> {
        self.init_step_timer()?;

        let mut msg = MSG::default();
        let window = self.get_game_window()?;

        loop {
            unsafe {
                if PeekMessageW(&mut msg, Some(window.platform.get_hwnd()), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                } else {
                    //TODO
                }

                if msg.message == WM_QUIT || msg.message == 0
                {
                    break;
                }
            }
        }

        Ok(())
    }

    fn tick(&mut self) -> XnaResult<()> {
        let mut timer = self.get_step_timer()?;

        let mut lambda = |timer: &StepTimer| -> XnaResult<()> {
            let elapsed = timer.get_elapsed_seconds();
            let total = timer.get_total_seconds();
            let elapsed_time = TimeSpan::from_seconds(elapsed as i32)?;
            let total_time = TimeSpan::from_seconds(total as i32)?;
            let game_time = GameTime {
                elapsed_time,
                total_time,
                is_slowly: false
            };

            self.set_game_time(&game_time)?;
            //TODO: self.update();

            Ok(())
        };

        timer.tick(&mut lambda)?;

        self.set_step_timer(timer)?;

        //TODO: implementar restante do código

        Ok(())
    }
}

impl Game {
    fn init_step_timer(&mut self) -> XnaResult<()> {
        let mut game = self.reference.get_mut()?;
        game.step_timer = StepTimer::new()?;

        Ok(())
    }

    fn set_game_time(&mut self, game_time: &GameTime) -> XnaResult<()> {
        let mut game = self.reference
            .try_get_mut(null_pointer_exception!())?;

        game.game_time = game_time.clone();

        Ok(())
    }

    pub fn get_game_window(&self) -> XnaResult<GameWindow> {
        let window = self.reference
            .try_get(null_pointer_exception!())?
            .game_window
            .clone();

        Ok(window)
    }

    pub fn get_step_timer(&self) -> XnaResult<StepTimer> {
        let timer = self.reference
            .try_get(null_pointer_exception!())?
            .step_timer
            .clone();

        Ok(timer)
    }

    pub fn set_step_timer(&mut self, step_timer: StepTimer) -> XnaResult<()> {
        let mut game = self.reference
            .try_get_mut(null_pointer_exception!())?;

        game.step_timer = step_timer;

        Ok(())
    }
}