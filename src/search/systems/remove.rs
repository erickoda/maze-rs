use bevy::prelude::*;

use crate::maze::MazeSquare;

pub fn remove_maze(commands: &mut Commands, query: &Query<Entity, With<MazeSquare>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
