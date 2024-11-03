use crate::maze::square_role::MazeSquareRole;

use super::utils::{find_maze_entry, find_maze_exit, get_empty_neighborhood, Position};

pub fn a_star(maze: Vec<Vec<MazeSquareRole>>) -> Option<Vec<Position>> {
    let mut found_path_to_exit_maze: Option<Vec<Position>> = None;
    let mut visited_paths_queue: Vec<Path> = Vec::new();

    let exit = find_maze_exit(maze.clone());
    let entry = find_maze_entry(maze.clone());
    visited_paths_queue.push(Path {
        positions: vec![entry.clone()],
        g: 0.0,
        h: heuristic_cost(entry.clone(), exit.clone()),
    });

    loop {
        // Remove and Get the top of the stack
        let actual_best_path = visited_paths_queue.pop();

        // Verify if actual best path exists
        if actual_best_path.is_none() {
            break;
        }

        let actual_best_path = actual_best_path.unwrap();

        // Get the last position of the actual best path
        let last_position = actual_best_path.positions[0].clone();

        // Verify if last position is the exit
        if last_position.x == exit.x && last_position.y == exit.y {
            found_path_to_exit_maze = Some(actual_best_path.positions);
            break;
        }

        // Get neighborhood
        let neighborhood = get_empty_neighborhood(maze.clone(), last_position);

        // Get neighborhood that wasn't visit in current path
        let not_visited_neighborhood = neighborhood
            .iter()
            .filter(|&neighbor_position| {
                !actual_best_path
                    .positions
                    .iter()
                    .any(|position| (*neighbor_position) == (*position))
            })
            .collect::<Vec<&Position>>();

        // Push the new neighborhood into the visited_queue
        for neighbor in not_visited_neighborhood {
            let current_best_path_clone = actual_best_path.clone();
            let mut current_best_positions = current_best_path_clone.positions.clone();

            current_best_positions.insert(0, neighbor.clone());

            let new_path = Path {
                positions: current_best_positions,
                g: current_best_path_clone.g + 1.,
                h: heuristic_cost(neighbor.clone(), exit.clone()),
            };

            visited_paths_queue.push(new_path);
        }

        // Sort the queue by cost in ascending order
        visited_paths_queue.sort_by(|a, b| a.cost().partial_cmp(&b.cost()).unwrap());
    }

    found_path_to_exit_maze
}

// calculate the manhattan distance between the current position and the exit
fn heuristic_cost(position: Position, exit: Position) -> f32 {
    let x = (position.x as f32 - exit.x as f32).abs();
    let y = (position.y as f32 - exit.y as f32).abs();

    x + y
}

#[derive(Clone, Debug)]
struct Path {
    positions: Vec<Position>, // path positions
    g: f32,                   // cost to reach this path
    h: f32,                   // heuristic cost to exit
}

impl Path {
    // Calculates the cost into current path
    fn cost(&self) -> f32 {
        self.g + self.h
    }
}
