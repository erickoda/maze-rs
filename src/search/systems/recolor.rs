use std::{collections::VecDeque, time::Duration};

use bevy::{
    color::{
        self,
        palettes::css::{BLUE, RED},
    },
    prelude::*,
};

use crate::maze::{MazeSquare, Position};

pub type PathWithColor = (VecDeque<Position>, Color);

#[derive(Resource)]
pub struct PendingColorUpdates {
    pub updates: VecDeque<PathWithColor>,
    pub timer: Timer,
}

pub fn spawn_pending_color_updates(mut commands: Commands) {
    commands.insert_resource(PendingColorUpdates {
        updates: VecDeque::new(),
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });
}

// Looks for pending updates and process them
pub fn process_pending_recolor_updates(
    mut pending_updates: ResMut<PendingColorUpdates>,
    mut table_query: Query<(&Position, &mut Handle<ColorMaterial>), With<MazeSquare>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    pending_updates.timer.tick(Duration::from_millis(100));

    if pending_updates.timer.finished() {
        if let Some(path_with_color) = pending_updates.updates.pop_front() {
            // for (color, position) in path {
            // println!("Processing color update for position: {:?}", position);
            recolor_table_squares(
                &mut table_query,
                &mut materials,
                path_with_color.0,
                path_with_color.1,
            );
            // }
        } else {
            println!("No pending updates to process.");
        }
    }
}

fn recolor_table_squares(
    table_query: &mut Query<(&Position, &mut Handle<ColorMaterial>), With<MazeSquare>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    path: VecDeque<Position>,
    new_color: Color,
) {
    let red_material = materials.add(Color::from(RED));
    let new_color_material = materials.add(new_color);

    for (square_position, mut material_handle) in table_query.iter_mut() {
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
