use crate::search::systems::recolor::components::maze_animation_speed::MazeAnimationSpeed;
use bevy::prelude::*;

pub fn spawn_maze_animation_speed_controller(mut commands: Commands) {
    commands.insert_resource(MazeAnimationSpeed(200));
}
