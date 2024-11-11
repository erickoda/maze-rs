use bevy::prelude::*;

use crate::{
    maze::MazeTable,
    search::{a_star::a_star, depth_first::depth_first_search},
};

use super::MazeTableTasksController;

pub fn animate_depth_first_path(maze_tasks_channel: Res<MazeTableTasksController>) {
    let maze_table =
        MazeTable::get_from_file("assets/Labirintos/100x100/exemplo_labirinto.txt".into());
    depth_first_search(maze_table, &maze_tasks_channel);
}

pub fn animate_a_star_path(maze_tasks_channel: Res<MazeTableTasksController>) {
    let maze_table =
        MazeTable::get_from_file("assets/Labirintos/100x100/exemplo_labirinto.txt".into());
    a_star(maze_table, &maze_tasks_channel);
}
