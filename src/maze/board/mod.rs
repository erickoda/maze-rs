pub mod path;
pub mod sizes;
pub mod square;

use super::{file::MazeFileReader, Position};
use bevy::prelude::Resource;
use path::Path;
use square::MazeBoardSquare;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default, Resource)]
pub struct MazeBoard(pub Vec<Vec<MazeBoardSquare>>);

impl MazeBoard {
    pub fn get_from_file(file_path: impl Into<String>) -> MazeBoard {
        let lines = MazeFileReader::read(file_path.into()).unwrap();
        let board = Vec::from(lines);

        MazeBoard(board)
    }

    pub fn get_empty_neighborhood(&self, position: &Position) -> Path {
        let mut empty_neighborhood: Path = VecDeque::new();
        let valid_neighbor_role = [MazeBoardSquare::Empty, MazeBoardSquare::Exit];

        if position.x > 0 && valid_neighbor_role.contains(&self.0[position.x - 1][position.y]) {
            let left = Position {
                x: position.x - 1,
                y: position.y,
            };

            empty_neighborhood.push_back(left);
        }

        if position.x < self.0.len() - 1
            && valid_neighbor_role.contains(&self.0[position.x + 1][position.y])
        {
            let right = Position {
                x: position.x + 1,
                y: position.y,
            };
            empty_neighborhood.push_back(right);
        }

        if position.y > 0 && valid_neighbor_role.contains(&self.0[position.x][position.y - 1]) {
            let up = Position {
                x: position.x,
                y: position.y - 1,
            };
            empty_neighborhood.push_back(up);
        }

        if position.y < self.0[0].len() - 1
            && valid_neighbor_role.contains(&self.0[position.x][position.y + 1])
        {
            let down = Position {
                x: position.x,
                y: position.y + 1,
            };
            empty_neighborhood.push_back(down);
        }

        empty_neighborhood
    }

    pub fn get_exit(&self) -> Option<Position> {
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, square) in row.iter().enumerate() {
                if *square == MazeBoardSquare::Exit {
                    return Some(Position {
                        x: row_index,
                        y: column_index,
                    });
                }
            }
        }

        None
    }

    pub fn get_entry(&self) -> Option<Position> {
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, square) in row.iter().enumerate() {
                if *square == MazeBoardSquare::Entry {
                    return Some(Position {
                        x: row_index,
                        y: column_index,
                    });
                }
            }
        }

        None
    }
}
