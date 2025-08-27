pub mod game;
pub mod step_timer;
pub mod game_window;

use crate::csharp::TimeSpan;
use crate::framework::game::game_window::WindowsGameWindow;
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

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub struct StepTimer {
    frequency: i64,
    last_time: i64,
    max_delta: u64,

    elapsed_ticks: u64,
    total_ticks: u64,
    left_over_ticks: u64,

    frame_count: u32,
    frames_per_second: u32,
    frames_this_second: u32,
    second_counter: u64,

    pub target_elapsed_ticks: u64,
    pub is_fixed_time_step: bool,
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct RefGame {
    game_window: GameWindow,
    step_timer: StepTimer,
    game_time: GameTime
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct Game {
    reference: Ptr<RefGame>
}