use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::csharp::TimeSpan;
use crate::framework::game::{Game, GameTime, GameWindow, RefGame, StepTimer};
use crate::{exception, null_pointer_exception};
use crate::shared::{Ptr, XnaResult, Exception, exception};


impl Game {
    pub fn new() -> Self {
        Self {
            reference: Ptr::new(RefGame::default()),
        }
    }

    pub fn run(&mut self) -> XnaResult<()> {
        if self.reference.is_null() {
            self.reference = Ptr::new(RefGame::default());
        }

        let mut window = GameWindow::default();
        let result = window.create();

        if result.is_err() {
            println!("{}", result.err().unwrap());
            return Err(exception!("", None));
            //TODO: remover isso aqui
        }

        self.set_game_window(&window)?;

        if self.get_is_running()? {
            return Err(exception!("Game already running.", None));
        }

        if !self.get_game_window()?.is_created() {
            return Err(exception!("Window is not created.", None));
        }

        self.initialize()?;
        self.set_is_running(true)?;

        self.start_game_loop()?;

        Ok(())
    }

    pub fn initialize(&mut self) -> XnaResult<()> {
        self.load_content()?;

        Ok(())
    }

    pub fn start_game_loop(&mut self) -> XnaResult<()> {
        self.init_step_timer()?;        
        
        if cfg!(target_os = "windows") {
            self.win_game_loop()?;
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
    
    pub fn load_content(&mut self) -> XnaResult<()> {
        Ok(())
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

    pub fn set_game_window(&mut self, game_window: &GameWindow) -> XnaResult<()> {
        let mut game = self.reference
            .try_get_mut(null_pointer_exception!())?;

        game.game_window = game_window.clone();

        Ok(())
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

    pub fn get_is_running(&self) -> XnaResult<bool> {
        let is_running = self.reference
            .try_get(null_pointer_exception!())?
            .is_running;

        Ok(is_running)
    }

    pub fn set_is_running(&mut self, is_running: bool) -> XnaResult<()> {
        let mut game = self.reference
            .try_get_mut(null_pointer_exception!())?;

        game.is_running = is_running;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl Game {
    fn win_game_loop(&mut self) -> XnaResult<()> {
        let mut msg = MSG::default();
        let window = self.get_game_window()?;

        loop {
            unsafe {
                if PeekMessageW(&mut msg, Some(window.platform.get_hwnd()), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                } else {
                    self.tick()?;
                }

                if msg.message == WM_QUIT || msg.message == 0
                {
                    break;
                }
            }
        }
        
        Ok(())
    }

}