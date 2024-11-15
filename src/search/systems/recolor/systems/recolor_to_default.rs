use crate::{
    maze::MazeSquare,
    user_interface::theme::maze_colors::{CURRENT, PATH, VISITED},
};
use bevy::prelude::*;

pub fn recolor_maze_paths_to_default(
    board_with_color_query: &mut Query<&mut Handle<ColorMaterial>, With<MazeSquare>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let default_color = materials.add(PATH);

    for mut material_handle in board_with_color_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            let is_path = material.color == CURRENT || material.color == VISITED;
            if is_path {
                *material_handle = default_color.clone();
            }
        }
    }
}
