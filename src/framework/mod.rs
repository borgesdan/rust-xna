pub mod game;
pub mod graphics;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}