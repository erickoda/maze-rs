pub mod a_star;
pub mod depth_first;
pub mod systems;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use systems::{
    animations::{animate_a_star_path, animate_depth_first_path},
    SearchSystemsPlugin,
};

pub struct SearchPlugin;

impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SearchSystemsPlugin)
            .add_systems(
                Update,
                animate_depth_first_path.run_if(input_just_pressed(KeyCode::KeyD)),
            )
            .add_systems(
                Update,
                animate_a_star_path.run_if(input_just_pressed(KeyCode::KeyA)),
            );
    }
}
