use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct MazeAnimationSpeed(pub u32);

impl From<MazeAnimationSpeed> for u32 {
    fn from(value: MazeAnimationSpeed) -> Self {
        if value.0 < MazeAnimationSpeed::get_min() {
            return MazeAnimationSpeed::get_min();
        }

        if value.0 > MazeAnimationSpeed::get_max() {
            return MazeAnimationSpeed::get_max();
        }

        value.0
    }
}

impl MazeAnimationSpeed {
    pub fn slower(&mut self) {
        if self.0 <= MazeAnimationSpeed::get_min() {
            return;
        }

        self.0 -= 10;
    }

    pub fn faster(&mut self) {
        if self.0 >= MazeAnimationSpeed::get_max() {
            return;
        }

        self.0 += 10;
    }

    pub fn get_max() -> u32 {
        251
    }

    pub fn get_min() -> u32 {
        1
    }
}
