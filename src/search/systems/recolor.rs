use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use crate::maze::{MazeSquare, Position};

#[derive(Resource)]
pub struct PendingColorUpdates {
    pub updates: VecDeque<(Color, Position)>,
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
        if let Some((color, position)) = pending_updates.updates.pop_front() {
            println!("Processing color update for position: {:?}", position);
            recolor_table_square(&mut table_query, &mut materials, &position, color);
        } else {
            println!("No pending updates to process.");
        }
    }
}

pub fn recolor_table_square(
    table_query: &mut Query<(&Position, &mut Handle<ColorMaterial>), With<MazeSquare>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: &Position,
    new_color: Color,
) {
    for (square_position, mut material_handle) in table_query.iter_mut() {
        if square_position == position {
            if let Some(material) = materials.get_mut(&*material_handle) {
                if material.color != new_color {
                    let new_material = materials.add(ColorMaterial::from(new_color));
                    *material_handle = new_material;
                }
            }
        }
    }
}
