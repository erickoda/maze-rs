use bevy::prelude::*;
use std::collections::VecDeque;

use crate::maze::board::path::PathWithColor;

#[derive(Resource)]
pub struct PendingColorUpdates {
    pub updates: VecDeque<PathWithColor>,
    pub timer: Timer,
}
