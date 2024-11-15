pub mod execute;
pub mod receiver;
pub mod send;
pub mod spawn_receiver;

use crate::maze::board::{path::PathWithColor, MazeBoard};
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use execute::execute_maze_board;
use spawn_receiver::spawn_maze_board_tasks_receiver;

pub struct MazeBoardTasksPlugin;

impl Plugin for MazeBoardTasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_maze_board_tasks_receiver)
            .add_systems(FixedUpdate, execute_maze_board);
    }
}

pub enum MazeBoardTasks {
    Create(MazeBoard),
    Destroy(),
    Update(PathWithColor),
    Clear(),
}

#[derive(Resource)]
pub struct MazeBoardTasksController {
    pub tx_control: Sender<MazeBoardTasks>,
    pub rx_update: Receiver<MazeBoardTasks>,
}
