use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, IsWindow, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT, WTS_CONSOLE_DISCONNECT};
use crate::framework::game::Game;
use crate::shared::XnaResult;

#[cfg(target_os = "windows")]
impl Game {
    pub(crate) fn win_game_loop(&mut self) -> XnaResult<()> {
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

                if msg.message == WM_QUIT
                    || msg.message == WTS_CONSOLE_DISCONNECT
                    || msg.message == 0
                    || !IsWindow(Some(window.platform.get_hwnd())).as_bool()
                {
                    break;
                }
            }
        }

        Ok(())
    }

}