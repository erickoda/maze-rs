use crate::maze::Position;
use bevy::color::Color;
use std::collections::VecDeque;

pub type PathWithColor = (Path, Color);
pub type Path = VecDeque<Position>;
