use std::fs::File;
use std::io::{self, BufRead};

use super::square_role::MazeSquareRole;

pub struct MazeFileReader;

pub struct MazeString(pub Vec<String>);

impl Default for MazeString {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl MazeFileReader {
    pub fn read(file_path: String) -> io::Result<MazeString> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut lines: MazeString = MazeString::default();

        for line in reader.lines() {
            let line = line?;
            lines.0.push(line);
        }

        Ok(lines)
    }
}

impl From<MazeString> for Vec<Vec<MazeSquareRole>> {
    fn from(maze_string: MazeString) -> Self {
        let mut map = Vec::new();

        for line in maze_string.0 {
            let mut row = Vec::new();

            for position in line.chars() {
                row.push(MazeSquareRole::from(position));
            }

            map.push(row);
        }

        map
    }
}
