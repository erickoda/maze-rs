use super::maze_board_tasks::{
    send::send_maze_board_background_task, MazeBoardTasks, MazeBoardTasksController,
};
use crate::{
    maze::board::MazeBoard,
    search::{a_star::a_star, depth_first::depth_first_search},
};
use bevy::prelude::*;

pub fn animate_depth_first_path(
    maze_tasks_channel: Res<MazeBoardTasksController>,
    maze_board: Res<MazeBoard>,
) {
    send_maze_board_background_task(&maze_tasks_channel, MazeBoardTasks::Clear());
    depth_first_search(maze_board.clone(), &maze_tasks_channel);
}

pub fn animate_a_star_path(
    maze_tasks_channel: Res<MazeBoardTasksController>,
    maze_board: Res<MazeBoard>,
) {
    send_maze_board_background_task(&maze_tasks_channel, MazeBoardTasks::Clear());
    a_star(maze_board.clone(), &maze_tasks_channel);
}
