use crate::{
    maze::{board::path::Path, MazeSquare, Position},
    user_interface::theme::maze_colors::{CURRENT, VISITED},
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn recolor_board_visited_paths(
    board_with_color_and_position_query: &mut Query<
        (&Position, &mut Handle<ColorMaterial>),
        With<MazeSquare>,
    >,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    path: Path,
    new_color: Color,
    skipped_visited_positions: HashSet<Position>,
) {
    let visited_default_color = materials.add(VISITED);
    let new_color_material = materials.add(new_color);
    let path_set: HashSet<Position> = path.into_iter().collect();

    for (square_position, mut material_handle) in board_with_color_and_position_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            if path_set.contains(&square_position) {
                let new_material = new_color_material.clone();
                *material_handle = new_material;
                continue;
            }

            if material.color == CURRENT {
                let new_material = visited_default_color.clone();
                *material_handle = new_material;
            }

            if skipped_visited_positions.contains(&square_position) {
                let new_material = visited_default_color.clone();
                *material_handle = new_material;
            }
        }
    }
}
