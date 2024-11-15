mod animation_speed_button;
mod choose_maze_button;
mod hover_button;
mod spawn_main_menu;
mod ui_visibility;

use animation_speed_button::{change_speed_bar_length, change_speed_of_search_animation};
use bevy::app::{App, Plugin, Startup, Update};
use choose_maze_button::choose_maze_button_system;
use hover_button::button_styled_when_mouse_interaction;
use spawn_main_menu::spawn_main_menu;
use ui_visibility::toggle_ui_visibility_when_press_escape;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(
                Update,
                (change_speed_bar_length, change_speed_of_search_animation),
            )
            .add_systems(Update, toggle_ui_visibility_when_press_escape)
            .add_systems(Update, choose_maze_button_system)
            .add_systems(Update, button_styled_when_mouse_interaction);
    }
}
