use crate::csharp::TimeSpan;
use crate::framework::game::{Game, MyGame, GameHandler, GameTime, GameWindow, RefGame, StepTimer};
use crate::{exception, null_pointer_exception};
use crate::shared::{Ptr, XnaResult};

impl Game {
    pub fn new(my_game: Box<dyn MyGame>) -> Self {
        Self {
            reference: Ptr::new(RefGame::default()),
            my_game,
        }
    }

    pub fn change(&mut self, my_game: Box<dyn MyGame>) {
        self.my_game = my_game
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

    pub fn start_game_loop(&mut self) -> XnaResult<()> {
        self.init_step_timer()?;        
        
        if cfg!(target_os = "windows") {
            self.win_game_loop()?;
        }        

        Ok(())
    }

    pub(crate) fn tick(&mut self) -> XnaResult<()> {
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
            self.update(&game_time)?;

            Ok(())
        };

        timer.tick(&mut lambda)?;
        self.set_step_timer(timer)?;

        self.draw(&self.get_game_time()?)?;

        Ok(())
    }

    pub fn initialize(&mut self) -> XnaResult<()> {
        let game = GameHandler {
            game: self.reference.clone(),
        };

        self.my_game.initialize(game)?;

        self.load_content()
    }

    pub fn load_content(&mut self) -> XnaResult<()> {
        let game = GameHandler {
            game: self.reference.clone(),
        };

        self.my_game.load_content(game)
    }
    
    pub fn update(&mut self, game_time: &GameTime) -> XnaResult<()> {
        let game = GameHandler {
            game: self.reference.clone(),
        };

        self.my_game.update(game_time.clone(), game)
    }

    pub fn draw(&mut self, game_time: &GameTime) -> XnaResult<()> {
        let game = GameHandler {
            game: self.reference.clone(),
        };

        self.my_game.draw(game_time.clone(), game)
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

    fn set_game_window(&mut self, game_window: &GameWindow) -> XnaResult<()> {
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

    fn set_step_timer(&mut self, step_timer: StepTimer) -> XnaResult<()> {
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

    fn set_is_running(&mut self, is_running: bool) -> XnaResult<()> {
        let mut game = self.reference
            .try_get_mut(null_pointer_exception!())?;

        game.is_running = is_running;

        Ok(())
    }

    pub fn get_game_time(&self) -> XnaResult<GameTime> {
        let game_time = self.reference
            .try_get(null_pointer_exception!())?
            .game_time;

        Ok(game_time)
    }
}