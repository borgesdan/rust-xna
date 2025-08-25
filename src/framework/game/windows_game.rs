use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, IsWindow, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::framework::game::Game;
use crate::null_pointer_exception;
use crate::shared::XnaResult;
use crate::shared::Exception;

impl Game {
    pub fn start_game_loop(&mut self) -> XnaResult<()> {
        let mut msg = MSG::default();
        let mut window = self.reference
            .try_get_mut(null_pointer_exception!())?
            .game_window
            .clone();


        loop {
            unsafe {
                if PeekMessageW(&mut msg, Some(window.platform.get_hwnd()), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                } else {
                    //TODO: not implemeted
                }

                if msg.message == WM_QUIT
                    || msg.message == 0
                //    || !IsWindow(Some(window.platform.get_hwnd())).as_bool()
                {
                    break;
                }
            }
        }

        Ok(())
    }
}