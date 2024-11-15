use std::collections::HashSet;
use std::time::Instant;
use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use crate::user_interface::theme::maze_colors::PATH;
use crate::{
    maze::{MazeSquare, Position},
    user_interface::theme::maze_colors::{CURRENT, VISITED},
};

pub type PathWithColor = (Path, Color);
pub type Path = VecDeque<Position>;

#[derive(Resource)]
pub struct PendingColorUpdates {
    pub updates: VecDeque<PathWithColor>,
    pub timer: Timer,
}

#[derive(Resource, Clone)]
pub struct MazeAnimationSpeed(u32);

impl From<MazeAnimationSpeed> for u32 {
    fn from(value: MazeAnimationSpeed) -> Self {
        if value.0 < MazeAnimationSpeed::get_min() {
            return MazeAnimationSpeed::get_min();
        }

        if value.0 > MazeAnimationSpeed::get_max() {
            return MazeAnimationSpeed::get_max();
        }

        value.0
    }
}

impl MazeAnimationSpeed {
    pub fn slower(&mut self) {
        if self.0 <= MazeAnimationSpeed::get_min() {
            return;
        }

        self.0 -= 10;
    }

    pub fn faster(&mut self) {

        if self.0 >= MazeAnimationSpeed::get_max() {
            return;
        }

        self.0 += 10;
    }

    pub fn get_max() -> u32 {
        251
    }

    pub fn get_min() -> u32 {
        1
    }
}

#[derive(Component)]
pub enum RecolorSpeedChanger {
    Slower,
    Faster,
}

pub fn spawn_quantity_of_skipped_path_on_recolor(mut commands: Commands) {
    commands.insert_resource(MazeAnimationSpeed(200));
}

pub fn spawn_pending_color_updates(mut commands: Commands) {
    commands.insert_resource(PendingColorUpdates {
        updates: VecDeque::new(),
        timer: Timer::from_seconds(0.00001, TimerMode::Repeating),
    });
}

pub fn process_pending_recolor_updates(
    mut pending_updates: ResMut<PendingColorUpdates>,
    mut table_with_color_and_position_query: Query<
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
        recolor_table_path(
            &mut table_with_color_and_position_query,
            &mut materials,
            path.0,
            path.1,
            skipped_visited_positions,
        );
        let duration = start.elapsed();
        println!("Recolor path took: {:?}", duration);
    }
}

pub fn recolor_maze_paths_to_default(
    table_with_color_query: &mut Query<&mut Handle<ColorMaterial>, With<MazeSquare>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let default_color = materials.add(PATH);

    for mut material_handle in table_with_color_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            let is_path = material.color == CURRENT || material.color == VISITED;
            if is_path {
                *material_handle = default_color.clone();
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
    skipped_visited_positions: HashSet<Position>,
) {
    let visited_default_color = materials.add(VISITED);
    let new_color_material = materials.add(new_color);
    let path_set: HashSet<Position> = path.into_iter().collect();

    for (square_position, mut material_handle) in table_with_color_and_position_query.iter_mut() {
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
