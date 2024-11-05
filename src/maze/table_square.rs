use bevy::prelude::Component;

#[derive(Clone, Debug, Component, PartialEq)]
pub enum MazeTableSquare {
    Empty,
    Wall,
    Entry,
    Exit,
    PathToExit,
}

impl MazeTableSquare {
    pub fn generate_random() -> Self {
        let random = rand::random::<usize>() % 2;

        match random {
            0 => MazeTableSquare::Empty,
            1 => MazeTableSquare::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<char> for MazeTableSquare {
    fn from(value: char) -> Self {
        match value {
            '0' => MazeTableSquare::Empty,
            '1' => MazeTableSquare::Wall,
            '2' => MazeTableSquare::Entry,
            '3' => MazeTableSquare::Exit,
            '4' => MazeTableSquare::PathToExit,
            _ => panic!("Invalid value for Maze"),
        }
    }
}
