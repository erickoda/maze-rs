mod file;
pub mod table_sizes;
pub mod table_square;

use std::collections::VecDeque;

use bevy::prelude::{Component, Resource};
use file::MazeFileReader;
use table_square::MazeTableSquare;

use crate::search::systems::recolor::Path;

#[derive(Debug, Clone, Default, Resource)]
pub struct MazeTable(pub Vec<Vec<MazeTableSquare>>);

#[derive(Component)]
pub struct MazeSquare;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Component, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl MazeTable {
    pub fn get_from_file(file_path: impl Into<String>) -> MazeTable {
        let lines = MazeFileReader::read(file_path.into()).unwrap();
        let table = Vec::from(lines);

        MazeTable(table)
    }

    pub fn get_empty_neighborhood(&self, position: &Position) -> Path {
        let mut empty_neighborhood: Path = VecDeque::new();
        let valid_neighbor_role = [MazeTableSquare::Empty, MazeTableSquare::Exit];

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
                if *square == MazeTableSquare::Exit {
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
                if *square == MazeTableSquare::Entry {
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
