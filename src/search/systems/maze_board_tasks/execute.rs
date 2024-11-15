use super::{MazeBoardTasks, MazeBoardTasksController};
use crate::{
    maze::MazeSquare,
    search::systems::{
        recolor::{
            resources::pending_colors_queue::PendingColorUpdates,
            systems::recolor_to_default::recolor_maze_paths_to_default,
        },
        remove::remove_maze,
        spawn::spawn_chosen_maze,
    },
};
use bevy::prelude::*;

pub fn execute_maze_board(
    maze_tasks_channel: Res<MazeBoardTasksController>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&Window>,
    mut pending_updates: ResMut<PendingColorUpdates>,
    destroy_query: Query<Entity, With<MazeSquare>>,
    mut board_with_color_and_position_query: Query<&mut Handle<ColorMaterial>, With<MazeSquare>>,
    asset_server: Res<AssetServer>,
) {
    while let Ok(msg) = maze_tasks_channel.rx_update.try_recv() {
        match msg {
            MazeBoardTasks::Create(maze_board) => {
                spawn_chosen_maze(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    maze_board.clone(),
                    &mut windows,
                    &asset_server,
                );
            }
            MazeBoardTasks::Update(path_with_color) => {
                pending_updates.updates.push_back(path_with_color);
            }
            MazeBoardTasks::Destroy() => {
                pending_updates.updates.clear();
                remove_maze(&mut commands, &destroy_query);
            }
            MazeBoardTasks::Clear() => {
                pending_updates.updates.clear();
                recolor_maze_paths_to_default(
                    &mut board_with_color_and_position_query,
                    &mut materials,
                );
            }
        }
    }
}
