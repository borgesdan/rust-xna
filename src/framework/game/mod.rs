#[cfg(feature = "windows")]
pub mod windows_game_window;
#[cfg(feature = "windows")]
use crate::framework::game::windows_game_window::WindowsGameWindow;
use crate::shared::Ptr;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct GameWindow {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub style: GameWindowStyle,

    x: i32,
    y: i32,
    is_fullscreen: bool,

    #[cfg(feature = "windows")]
    platform: WindowsGameWindow,
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum GameWindowStyle {
    #[default]
    Windowed,
    FullScreen,
    BorderlessFullScreen
}

pub struct RefGame {

}

pub struct Game {
    reference: Ptr<RefGame>
}