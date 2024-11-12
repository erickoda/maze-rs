use bevy::prelude::*;

use crate::{
    maze::MazeTable,
    search::{a_star::a_star, depth_first::depth_first_search},
};

use super::{send_maze_table_background_task, MazeTableTasks, MazeTableTasksController};

pub fn animate_depth_first_path(
    maze_tasks_channel: Res<MazeTableTasksController>,
    maze_table: Res<MazeTable>,
) {
    send_maze_table_background_task(&maze_tasks_channel, MazeTableTasks::Clear());
    depth_first_search(maze_table.clone(), &maze_tasks_channel);
}

pub fn animate_a_star_path(
    maze_tasks_channel: Res<MazeTableTasksController>,
    maze_table: Res<MazeTable>,
) {
    send_maze_table_background_task(&maze_tasks_channel, MazeTableTasks::Clear());
    a_star(maze_table.clone(), &maze_tasks_channel);
}
