use bevy::{
    color::palettes::css::{BLACK, RED},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    maze::{table_square::MazeTableSquare, MazeSquare, MazeTable, Position},
    user_interface::theme::{COMPLEMENTARY_200, NEUTRAL_0, PRIMARY_200},
};

pub fn spawn_chosen_maze(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    maze_table: MazeTable,
    windows: &mut Query<&Window>,
) {
    let window = windows.single();
    let window_width = window.resolution.width();

    let scale = window_width / maze_table.0.len() as f32;
    let width = scale * maze_table.0.len() as f32;
    let height = scale * maze_table.0.len() as f32;
    let square_mesh = meshes.add(Rectangle::default());
    let material_empty = materials.add(PRIMARY_200);
    let material_wall = materials.add(Color::from(BLACK));
    let material_entry = materials.add(NEUTRAL_0);
    let material_exit = materials.add(Color::from(RED));
    let material_path = materials.add(COMPLEMENTARY_200);

    for (i, row) in maze_table.0.iter().enumerate() {
        for (j, square_role) in row.clone().iter().enumerate() {
            let translation = Vec3::new(
                i as f32 * scale - width / 2.,
                j as f32 * scale - height / 2.,
                1.0,
            );

            let material = match square_role {
                MazeTableSquare::Empty => material_empty.clone(),
                MazeTableSquare::Wall => material_wall.clone(),
                MazeTableSquare::Entry => material_entry.clone(),
                MazeTableSquare::Exit => material_exit.clone(),
                MazeTableSquare::PathToExit => material_path.clone(),
            };

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: square_mesh.clone().into(),
                    material: material.clone(),
                    transform: Transform::from_translation(translation)
                        .with_scale(Vec3::splat(scale)),
                    ..Default::default()
                },
                MazeSquare,
                Position { x: i, y: j },
            ));
        }
    }
}
