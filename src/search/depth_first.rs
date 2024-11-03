use crate::maze::square_role::MazeSquareRole;

use super::utils::{find_maze_entry, find_maze_exit, get_empty_neighborhood, Position};

pub fn depth_first_search(maze: Vec<Vec<MazeSquareRole>>) -> Option<Vec<Position>> {
    let mut found_path_to_exit_maze: Option<Vec<Position>> = None;
    let mut visited_paths_stack: Vec<Vec<Position>> = Vec::new();

    let exit = find_maze_exit(maze.clone());
    let entry = vec![find_maze_entry(maze.clone())];
    visited_paths_stack.push(entry.clone());

    loop {
        // Verify if there are nodes that weren't visited
        if visited_paths_stack.is_empty() {
            break;
        }

        // Remove and get the top element of the visited stack to search into it
        let last_visited_path = visited_paths_stack.remove(0);

        // Get the top element of the current path
        let last_visited_position = &last_visited_path[0];

        // Get the neighborhood of the chosen position
        let neighborhood = get_empty_neighborhood(maze.clone(), last_visited_position.clone());

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
    }

    found_path_to_exit_maze
}
