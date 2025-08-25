#[cfg(target_os = "windows")]
pub mod windows_game_window;
#[cfg(target_os = "windows")]
pub mod windows_game;

mod game;

use crate::csharp::TimeSpan;
#[cfg(target_os = "windows")]
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

    #[cfg(target_os = "windows")]
    platform: WindowsGameWindow,
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum GameWindowStyle {
    #[default]
    Windowed,
    FullScreen,
    BorderlessFullScreen
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub struct GameTime {
    pub elapsed_time: TimeSpan,
    pub is_slowly: bool,
    pub total_time: TimeSpan,
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct RefGame {
    game_window: GameWindow,
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct Game {
    reference: Ptr<RefGame>
}