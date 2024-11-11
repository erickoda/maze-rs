use std::time::Duration;

use bevy::{
    color::palettes::{
        css::{BLUE, RED},
        tailwind::BLUE_200,
    },
    prelude::*,
};

use crate::maze::{MazeSquare, Position};

pub type PathWithColor = (Path, Color);
pub type Path = Vec<Position>;

#[derive(Resource)]
pub struct PendingColorUpdates {
    pub updates: Vec<PathWithColor>,
    pub timer: Timer,
}

pub fn spawn_pending_color_updates(mut commands: Commands) {
    commands.insert_resource(PendingColorUpdates {
        updates: Vec::new(),
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });
}

pub fn process_pending_recolor_updates(
    mut pending_updates: ResMut<PendingColorUpdates>,
    mut table_with_color_and_position_query: Query<
        (&Position, &mut Handle<ColorMaterial>),
        With<MazeSquare>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    pending_updates.timer.tick(Duration::from_millis(100));

    if pending_updates.updates.is_empty() {
        return;
    }

    if !pending_updates.timer.finished() {
        return;
    }

    let path_with_color = pending_updates.updates.remove(0);
    recolor_table_path(
        &mut table_with_color_and_position_query,
        &mut materials,
        path_with_color.0,
        path_with_color.1,
    );
}

pub fn recolor_all_visited_nodes_to_default(
    mut table_with_colors_query: Query<
        &mut Handle<ColorMaterial>,
        (With<MazeSquare>, With<Position>),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let default_empty_material = materials.add(Color::from(BLUE_200));

    for mut material_handle in table_with_colors_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            let is_painted =
                material.color == Color::from(BLUE) || material.color == Color::from(RED);

            if is_painted {
                let new_material = default_empty_material.clone();
                *material_handle = new_material;
            }
        }
    }
}

fn recolor_table_path(
    table_with_color_and_position_query: &mut Query<
        (&Position, &mut Handle<ColorMaterial>),
        With<MazeSquare>,
    >,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    path: Path,
    new_color: Color,
) {
    let red_material = materials.add(Color::from(RED));
    let new_color_material = materials.add(new_color);

    for (square_position, mut material_handle) in table_with_color_and_position_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            if material.color == Color::from(BLUE) {
                let new_material = red_material.clone();
                *material_handle = new_material;
            }
        }

        for position in path.iter() {
            if square_position == position {
                if let Some(material) = materials.get_mut(&*material_handle) {
                    if material.color != new_color {
                        let new_material = new_color_material.clone();
                        *material_handle = new_material;
                    }
                }
            }
        }
    }
}
