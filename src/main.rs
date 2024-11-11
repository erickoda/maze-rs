mod camera;
mod maze;
mod search;
mod user_interface;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use camera::{move_camera, spawn_camera};
use search::systems::animations::{animate_a_star_path, animate_depth_first_path};
use search::systems::recolor::{process_pending_recolor_updates, spawn_pending_color_updates};
use search::systems::{execute_maze_table_tasks, spawn_maze_table_tasks_receiver};
use user_interface::spawn_user_menu;
use user_interface::system::chosen_maze::button::choose_maze_button_system;
use user_interface::system::ui_visibility::toggle_ui_visibility_when_press_escape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_user_menu))
        .add_systems(
            Startup,
            (spawn_maze_table_tasks_receiver, spawn_pending_color_updates),
        )
        .add_systems(Update, move_camera)
        .add_systems(Update, choose_maze_button_system)
        .add_systems(Update, toggle_ui_visibility_when_press_escape)
        .add_systems(
            Update,
            animate_depth_first_path.run_if(input_just_pressed(KeyCode::KeyD)),
        )
        .add_systems(
            Update,
            animate_a_star_path.run_if(input_just_pressed(KeyCode::KeyA)),
        )
        .add_systems(FixedUpdate, execute_maze_table_tasks)
        .add_systems(Update, process_pending_recolor_updates) // Adiciona o novo sistema
        .run();
}
