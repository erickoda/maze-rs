use bevy::color::Color;

use super::{COMPLEMENTARY_200, PRIMARY_200, PRIMARY_600};

pub const ENTRY: Color = PRIMARY_200;
pub const EXIT: Color = COMPLEMENTARY_200;
pub const WALL: Color = PRIMARY_600;
pub const PATH: Color = Color::WHITE;
pub const VISITED: Color = PRIMARY_200;
pub const CURRENT: Color = COMPLEMENTARY_200;
