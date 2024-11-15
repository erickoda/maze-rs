use crate::search::systems::recolor::resources::pending_colors_queue::PendingColorUpdates;
use bevy::prelude::*;
use std::collections::VecDeque;

pub fn spawn_pending_color_queue(mut commands: Commands) {
    commands.insert_resource(PendingColorUpdates {
        updates: VecDeque::new(),
        timer: Timer::from_seconds(0.00001, TimerMode::Repeating),
    });
}
