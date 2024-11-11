use bevy::prelude::*;

use crate::{
    maze::MazeTable,
    search::{a_star::a_star, depth_first::depth_first_search},
};

use super::MazeTableTasksController;

pub fn animate_depth_first_path(
    maze_tasks_channel: Res<MazeTableTasksController>,
    maze_table: Res<MazeTable>,
) {
    depth_first_search(maze_table.clone(), &maze_tasks_channel);
}

pub fn animate_a_star_path(
    maze_tasks_channel: Res<MazeTableTasksController>,
    maze_table: Res<MazeTable>,
) {
    a_star(maze_table.clone(), &maze_tasks_channel);
}
