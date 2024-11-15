use bevy::prelude::Component;

#[derive(Clone, Debug, Component, PartialEq)]
pub enum MazeBoardSquare {
    Empty,
    Wall,
    Entry,
    Exit,
    PathToExit,
}

impl MazeBoardSquare {
    pub fn generate_random() -> Self {
        let random = rand::random::<usize>() % 2;

        match random {
            0 => MazeBoardSquare::Empty,
            1 => MazeBoardSquare::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<char> for MazeBoardSquare {
    fn from(value: char) -> Self {
        match value {
            '0' => MazeBoardSquare::Empty,
            '1' => MazeBoardSquare::Wall,
            '2' => MazeBoardSquare::Entry,
            '3' => MazeBoardSquare::Exit,
            '4' => MazeBoardSquare::PathToExit,
            _ => panic!("Invalid value for Maze"),
        }
    }
}
