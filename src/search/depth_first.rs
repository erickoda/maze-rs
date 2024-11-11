use bevy::prelude::*;

use crate::{
    maze::{MazeTable, Position},
    user_interface::theme::maze_colors::CURRENT,
};

use super::systems::{
    recolor::Path, send_maze_table_background_task, MazeTableTasks, MazeTableTasksController,
};

pub fn depth_first_search(
    maze_table: MazeTable,
    maze_tasks_channel: &Res<MazeTableTasksController>,
) -> Option<Path> {
    let mut found_path_to_exit_maze: Option<Path> = None;
    let mut visited_paths_stack: Vec<Path> = Vec::new();

    let exit = maze_table.get_exit();
    let entry = maze_table.get_entry();

    if exit.is_none() {
        println!("The maze should have an Exit");
        return None;
    }

    if entry.is_none() {
        println!("The maze should have an Entry");
        return None;
    }

    let exit = exit.unwrap();
    let entry = entry.unwrap();

    visited_paths_stack.push(vec![entry.clone()]);

    loop {
        // Verify if there are nodes that weren't visited
        if visited_paths_stack.is_empty() {
            break;
        }

        // Remove and get the top element of the visited stack to search into it
        let last_visited_path = visited_paths_stack.remove(0);

        // Get the top element of the current path
        let last_visited_position = &last_visited_path[0];

        // Color the already searched path
        send_maze_table_background_task(
            maze_tasks_channel,
            MazeTableTasks::Update((last_visited_path.clone(), CURRENT)),
        );

        // Get the neighborhood of the chosen position
        let neighborhood = maze_table.get_empty_neighborhood(&last_visited_position);

        // Filter the neighborhood to find if some was already visit
        let not_visited_neighborhood = neighborhood
            .iter()
            .filter(|&neighborhood| {
                !last_visited_path
                    .iter()
                    .any(|position| *neighborhood == *position)
            })
            .collect::<Vec<&Position>>();

        // Insert the new neighbors into the stack
        for neighbor in not_visited_neighborhood {
            let mut last_visited_path_clone = last_visited_path.clone();
            last_visited_path_clone.insert(0, neighbor.clone());

            // Verify if this path is the exit
            if neighbor.x == exit.x && neighbor.y == exit.y {
                found_path_to_exit_maze = Some(last_visited_path_clone);
                break;
            }

            visited_paths_stack.insert(0, last_visited_path_clone);
        }

        // If the exit was found, break the loop
        if found_path_to_exit_maze.is_some() {
            break;
        }
    }

    found_path_to_exit_maze
}
