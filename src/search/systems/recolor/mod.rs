use bevy::app::{App, Plugin, Startup, Update};
use systems::{
    process_pending_colors_queue::process_pending_recolor_queue,
    spawn::{
        maze_animation_speed_controller::spawn_maze_animation_speed_controller,
        pending_color_queue::spawn_pending_color_queue,
    },
};

pub mod components;
pub mod resources;
pub mod systems;

pub struct MazeRecolorPlugin;

impl Plugin for MazeRecolorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_maze_animation_speed_controller,
                spawn_pending_color_queue,
            ),
        )
        .add_systems(Update, process_pending_recolor_queue);
    }
}
