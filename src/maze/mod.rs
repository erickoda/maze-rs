pub mod board;
mod file;

use bevy::prelude::Component;

#[derive(Component)]
pub struct MazeSquare;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Component, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
