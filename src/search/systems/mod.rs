pub mod animations;
pub mod maze_board_tasks;
pub mod recolor;
mod remove;
mod spawn;

use bevy::prelude::*;
use maze_board_tasks::MazeBoardTasksPlugin;
use recolor::MazeRecolorPlugin;

pub struct SearchSystemsPlugin;

impl Plugin for SearchSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MazeRecolorPlugin)
            .add_plugins(MazeBoardTasksPlugin);
    }
}
