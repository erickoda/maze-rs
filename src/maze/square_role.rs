use bevy::prelude::Component;

#[derive(Clone, Debug, Component, PartialEq)]
pub enum MazeSquareRole {
    Empty,
    Wall,
    Entry,
    Exit,
    PathToExit,
}

impl MazeSquareRole {
    pub fn generate_random() -> Self {
        let random = rand::random::<usize>() % 2;

        match random {
            0 => MazeSquareRole::Empty,
            1 => MazeSquareRole::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<char> for MazeSquareRole {
    fn from(value: char) -> Self {
        match value {
            '0' => MazeSquareRole::Empty,
            '1' => MazeSquareRole::Wall,
            '2' => MazeSquareRole::Entry,
            '3' => MazeSquareRole::Exit,
            '4' => MazeSquareRole::PathToExit,
            _ => panic!("Invalid value for Maze"),
        }
    }
}
