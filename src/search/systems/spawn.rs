use crate::{
    maze::{board::square::MazeBoardSquare, board::MazeBoard, MazeSquare, Position},
    user_interface::theme::{
        maze_colors::{ENTRY, EXIT, PATH, VISITED, WALL},
        COMPLEMENTARY_300,
    },
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::cmp::Ordering;

pub fn spawn_chosen_maze(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    maze_board: MazeBoard,
    windows: &mut Query<&Window>,
    asset_server: &Res<AssetServer>,
) {
    let window = windows.single();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();
    let min_window_size = window_width.min(window_height);

    let scale = {
        match min_window_size.partial_cmp(&window_height) {
            Some(Ordering::Equal) => min_window_size / (maze_board.0.len() as f32) * 0.60,
            _ => min_window_size / (maze_board.0.len() as f32) * 0.85,
        }
    };

    let width = scale * (maze_board.0.len() - 1) as f32;
    let height = scale * (maze_board.0.len() - 1) as f32;
    let square_mesh = meshes.add(Rectangle::default());
    let material_empty = materials.add(PATH);
    let material_wall = materials.add(WALL);
    let material_entry = materials.add(ENTRY);
    let material_exit = materials.add(EXIT);
    let material_path = materials.add(VISITED);

    commands.insert_resource(maze_board.clone());

    for (i, row) in maze_board.0.iter().enumerate() {
        for (j, square_role) in row.clone().iter().enumerate() {
            let translation = Vec3::new(
                i as f32 * scale - width / 2.,
                j as f32 * scale - height / 2.,
                1.0,
            );

            let material = match square_role {
                MazeBoardSquare::Empty => material_empty.clone(),
                MazeBoardSquare::Wall => material_wall.clone(),
                MazeBoardSquare::Entry => material_entry.clone(),
                MazeBoardSquare::Exit => material_exit.clone(),
                MazeBoardSquare::PathToExit => material_path.clone(),
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

    commands.spawn(
        TextBundle::from_section(
            "Press 󰬈 to display A* algorithm\nPress 󰬋 for DFS algorithm\nPress 󱊷 to open the menu\nPress 󰛀 󰛁 󰛂 󰜯 󰐖 󰍵 to movement camera
            ",
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono/JetBrainsMonoNerdFont-Thin.ttf"),
                font_size: 16.0,
                color: COMPLEMENTARY_300,
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.),
            right: Val::Px(0.),
            left: Val::Px(0.),
            ..default()
        }),
    );
}
