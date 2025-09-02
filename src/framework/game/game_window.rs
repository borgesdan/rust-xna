use crate::framework::game::{GameWindow, GameWindowStyle};
use crate::framework::game::win_game_window::{PlatformGameWindow};
use crate::shared::XnaResult;

impl GameWindow {
    ///Close the window.
    pub fn close(&self) -> XnaResult<()> {
        if cfg!(target_os = "windows") {
            PlatformGameWindow::close(self.platform.hwnd)?
        }

        Ok(())
    }

    pub fn create(&mut self) -> XnaResult<()> {
        self.sanitize();

        if cfg!(target_os = "windows") {
            let result = PlatformGameWindow::create(self)?;
            self.platform.hwnd = result.0;
            self.x = result.1;
            self.y = result.2;
        }

        Ok(())
    }

    fn sanitize(&mut self)  {
        if self.width == 0 {
            self.width = 800;
        }

        if self.height == 0 {
            self.height = 480;
        }
    }

    pub fn update(&mut self) -> XnaResult<()> {
        if self.style == GameWindowStyle::Windowed {
            if cfg!(target_os = "windows") {
                let result = PlatformGameWindow::update(self.platform.hwnd, self)?;
                self.x = result.0;
                self.y = result.1;
            }

        }

        Ok(())
    }

    pub fn is_created(&self) -> bool {
        !self.platform.hwnd.is_invalid()
    }
}