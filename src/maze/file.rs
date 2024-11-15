use std::fs::File;
use std::io::{self, BufRead};

use super::board::square::MazeBoardSquare;

pub struct MazeFileReader;

#[derive(Default, Debug)]
pub struct MazeString(pub Vec<String>);

impl MazeFileReader {
    pub fn read(file_path: impl Into<String>) -> io::Result<MazeString> {
        let file = File::open(file_path.into())?;
        let reader = io::BufReader::new(file);
        let mut lines: MazeString = MazeString::default();

        for line in reader.lines() {
            let line = line?;
            lines.0.push(line);
        }

        Ok(lines)
    }
}

impl From<MazeString> for Vec<Vec<MazeBoardSquare>> {
    fn from(maze_string: MazeString) -> Self {
        let mut map = Vec::new();
        let mut transpose_map = Vec::new();

        for line in maze_string.0 {
            let mut row = Vec::new();

            for position in line.chars() {
                row.push(MazeBoardSquare::from(position));
            }

            map.push(row);
        }

        // needs to transpose the map matrix and reverse each new line
        // because of how bevy 2D coordinates work
        for j in 0..map[0].len() {
            for i in 0..map.len() {
                if j >= transpose_map.len() {
                    transpose_map.push(Vec::new());
                }

                transpose_map[j].push(map[i][j].clone());
            }
            transpose_map[j].reverse();
        }

        transpose_map
    }
}
