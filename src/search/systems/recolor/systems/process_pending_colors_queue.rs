use crate::maze::{MazeSquare, Position};
use crate::search::systems::recolor::components::maze_animation_speed::MazeAnimationSpeed;
use crate::search::systems::recolor::resources::pending_colors_queue::PendingColorUpdates;
use crate::search::systems::recolor::systems::recolor_visited_paths::recolor_board_visited_paths;
use bevy::prelude::*;
use std::collections::HashSet;
use std::time::Instant;
use std::{collections::VecDeque, time::Duration};

pub fn process_pending_recolor_queue(
    mut pending_updates: ResMut<PendingColorUpdates>,
    mut board_with_color_and_position_query: Query<
        (&Position, &mut Handle<ColorMaterial>),
        With<MazeSquare>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    quantity_of_skipped_paths: Res<MazeAnimationSpeed>,
) {
    pending_updates.timer.tick(Duration::from_micros(10));

    if !pending_updates.timer.finished() {
        return;
    }

    let mut current_path_with_color: Option<(VecDeque<Position>, Color)> = None;
    let mut skipped_visited_positions: HashSet<Position> = HashSet::new();

    for _ in 0..quantity_of_skipped_paths.0 {
        if let Some(path_with_color) = pending_updates.updates.pop_front() {
            for position in &path_with_color.0 {
                skipped_visited_positions.insert(position.clone());
            }
            current_path_with_color = Some(path_with_color);
        }
    }

    if let Some(path) = current_path_with_color {
        let start = Instant::now();
        recolor_board_visited_paths(
            &mut board_with_color_and_position_query,
            &mut materials,
            path.0,
            path.1,
            skipped_visited_positions,
        );
        let duration = start.elapsed();
        println!("Recolor path took: {:?}", duration);
    }
}
