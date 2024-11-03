use crate::maze::square_role::MazeSquareRole;

pub fn find_maze_exit(maze: Vec<Vec<MazeSquareRole>>) -> Position {
    for (row_index, row) in maze.iter().enumerate() {
        for (column_index, square) in row.iter().enumerate() {
            if *square == MazeSquareRole::Exit {
                return Position {
                    x: row_index,
                    y: column_index,
                };
            }
        }
    }

    panic!("Maze entry not found");
}

pub fn find_maze_entry(maze: Vec<Vec<MazeSquareRole>>) -> Position {
    for (row_index, row) in maze.iter().enumerate() {
        for (column_index, square) in row.iter().enumerate() {
            if *square == MazeSquareRole::Entry {
                return Position {
                    x: row_index,
                    y: column_index,
                };
            }
        }
    }

    panic!("Maze entry not found");
}

pub fn get_empty_neighborhood(maze: Vec<Vec<MazeSquareRole>>, position: Position) -> Vec<Position> {
    let mut empty_neighborhood = Vec::new();
    let valid_neighbor_role = [MazeSquareRole::Empty, MazeSquareRole::Exit];

    if position.x > 0 && valid_neighbor_role.contains(&maze[position.x - 1][position.y]) {
        let left = Position {
            x: position.x - 1,
            y: position.y,
        };

        empty_neighborhood.push(left);
    }

    if position.x < maze.len() - 1
        && valid_neighbor_role.contains(&maze[position.x + 1][position.y])
    {
        let right = Position {
            x: position.x + 1,
            y: position.y,
        };
        empty_neighborhood.push(right);
    }

    if position.y > 0 && valid_neighbor_role.contains(&maze[position.x][position.y - 1]) {
        let up = Position {
            x: position.x,
            y: position.y - 1,
        };
        empty_neighborhood.push(up);
    }

    if position.y < maze[0].len() - 1
        && valid_neighbor_role.contains(&maze[position.x][position.y + 1])
    {
        let down = Position {
            x: position.x,
            y: position.y + 1,
        };
        empty_neighborhood.push(down);
    }

    empty_neighborhood
}

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
