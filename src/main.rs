mod camera;
mod components;
mod maze;
mod search;
mod user_interface;

use bevy::prelude::*;
use camera::{move_camera, spawn_camera};
use user_interface::spawn_user_menu;
use user_interface::system::chosen_maze::button::choose_maze_button_system;
use user_interface::system::ui_visibility::toggle_ui_visibility_when_press_escape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_user_menu))
        .add_systems(Update, move_camera)
        .add_systems(Update, choose_maze_button_system)
        .add_systems(Update, toggle_ui_visibility_when_press_escape)
        .run();
}
