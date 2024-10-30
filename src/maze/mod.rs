mod file;

use crossterm::cursor::position;
use file::{MazeFileReader, MazeString};
use rand::prelude::*;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Maze {
    pub size: usize,
    pub map: Vec<Vec<PositionRole>>,
}

#[derive(Clone, Debug)]
pub enum PositionRole {
    Empty,
    Wall,
    Entry,
    Exit,
}

impl PositionRole {
    fn generate_random() -> Self {
        let random = rand::random::<usize>() % 2;

        match random {
            0 => PositionRole::Empty,
            1 => PositionRole::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<PositionRole> for Color {
    fn from(value: PositionRole) -> Self {
        match value {
            PositionRole::Empty => Color::White,
            PositionRole::Wall => Color::DarkGray,
            PositionRole::Entry => Color::Magenta,
            PositionRole::Exit => Color::Red,
        }
    }
}

impl From<char> for PositionRole {
    fn from(value: char) -> Self {
        match value {
            '0' => PositionRole::Empty,
            '1' => PositionRole::Wall,
            '2' => PositionRole::Entry,
            '3' => PositionRole::Exit,
            _ => panic!("Invalid value for Maze"),
        }
    }
}

pub struct MazeBuilder {
    size: Option<usize>,
    map: Option<Vec<Vec<PositionRole>>>,
}

impl Default for MazeBuilder {
    fn default() -> Self {
        Self {
            size: None,
            map: None,
        }
    }
}

impl MazeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = Some(size);
    }

    pub fn set_map(&mut self, map: Vec<Vec<PositionRole>>) {
        self.map = Some(map);
    }

    pub fn build(self) -> Maze {
        let size = self.size.unwrap_or_default();
        let map = self.map.unwrap_or_default();

        Maze { size, map }
    }
}

impl Default for Maze {
    fn default() -> Self {
        Self {
            size: usize::default(),
            map: Vec::default(),
        }
    }
}

impl Maze {
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn generate(&mut self) {
        let size = self.size;

        let mut map = vec![vec![PositionRole::Empty; size]; size];

        for row in map.iter_mut() {
            for position in row.iter_mut() {
                *position = PositionRole::generate_random();
            }
        }

        if !self.verify_map() {
            return self.generate();
        }

        let entry = self.generate_random_position();
        let exit = self.generate_random_position();

        map[0][entry] = PositionRole::Entry;
        map[size - 1][exit] = PositionRole::Exit;

        self.map = map;
    }

    fn generate_random_position(&self) -> usize {
        let size = self.size;
        let random_position = rand::random::<usize>() % self.size;
        random_position
    }

    fn verify_map(&self) -> bool {
        true
    }

    pub fn get_from_file(file_path: String) -> Maze {
        let lines = MazeFileReader::read(file_path).unwrap();
        let map = Vec::from(lines);

        Maze {
            size: map.len(),
            map,
        }
    }
}
