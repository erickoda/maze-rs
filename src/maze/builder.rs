use super::{square_role::MazeSquareRole, Maze};

pub struct MazeBuilder {
    size: Option<usize>,
    map: Option<Vec<Vec<MazeSquareRole>>>,
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

    pub fn set_map(&mut self, map: Vec<Vec<MazeSquareRole>>) {
        self.map = Some(map);
    }

    pub fn build(self) -> Maze {
        let size = self.size.unwrap_or_default();
        let map = self.map.unwrap_or_default();

        Maze { size, map }
    }
}
