mod builder;
mod file;
pub mod square_role;

use bevy::prelude::Component;
use file::MazeFileReader;
use square_role::MazeSquareRole;

#[derive(Debug, Clone, Component)]
pub struct Maze {
    pub size: usize,
    pub map: Vec<Vec<MazeSquareRole>>,
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

        let mut map = vec![vec![MazeSquareRole::Empty; size]; size];

        for row in map.iter_mut() {
            for position in row.iter_mut() {
                *position = MazeSquareRole::generate_random();
            }
        }

        if !self.verify_map() {
            return self.generate();
        }

        let entry = self.generate_random_position();
        let exit = self.generate_random_position();

        map[0][entry] = MazeSquareRole::Entry;
        map[size - 1][exit] = MazeSquareRole::Exit;

        self.map = map;
    }

    fn generate_random_position(&self) -> usize {
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
