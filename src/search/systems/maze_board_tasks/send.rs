use super::{MazeBoardTasks, MazeBoardTasksController};
use bevy::prelude::*;

pub fn send_maze_board_background_task(
    maze_tasks_channel: &Res<MazeBoardTasksController>,
    recolor_message: MazeBoardTasks,
) {
    if let Err(e) = maze_tasks_channel.tx_control.try_send(recolor_message) {
        println!("Failed to send message to the net task: {:?}", e);
    }
}
