mod components;
mod maze;
mod search;

use bevy::{
    color::palettes::css::{BLACK, BLUE, PURPLE, RED, WHITE},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::MazeSquare;
use maze::{square_role::MazeSquareRole, Maze};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, load_maze))
        .run();
}

fn load_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut maze = Maze::get_from_file("Labirintos/200x200/exemplo_labirinto.txt".into());
    let a_star_path = search::a_star::a_star(maze.clone().map);

    if let Some(path) = a_star_path {
        // Predefine mesh and materials
        let square_mesh = meshes.add(Rectangle::default());
        let material_empty = materials.add(Color::from(WHITE));
        let material_wall = materials.add(Color::from(BLACK));
        let material_entry = materials.add(Color::from(BLUE));
        let material_exit = materials.add(Color::from(RED));
        let material_path = materials.add(Color::from(PURPLE));

        let scale = 50.0 / maze.size as f32 + 3.0;

        for i in 0..maze.clone().map.len() {
            for j in 0..maze.clone().map[i].len() {
                for position in path.iter() {
                    if i == position.x && j == position.y {
                        maze.map[i][j] = MazeSquareRole::PathToExit;
                    }
                }
            }
        }

        for (i, row) in maze.map.iter().enumerate() {
            for (j, square_role) in row.clone().iter().enumerate() {
                let position = Vec3::new(i as f32 * scale, j as f32 * scale, 1.0);
                let material = match square_role {
                    MazeSquareRole::Empty => material_empty.clone(),
                    MazeSquareRole::Wall => material_wall.clone(),
                    MazeSquareRole::Entry => material_entry.clone(),
                    MazeSquareRole::Exit => material_exit.clone(),
                    MazeSquareRole::PathToExit => material_path.clone(),
                };

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: square_mesh.clone().into(),
                        material,
                        transform: Transform::from_translation(position)
                            .with_scale(Vec3::splat(scale)),
                        ..Default::default()
                    },
                    MazeSquare,
                ));
            }
        }
    }

    commands.spawn(maze);
}

fn spawn_camera(mut commands: Commands) {
    let position = Vec3::new(0 as f32, 0 as f32, 0.);
    commands.spawn(Camera2dBundle::default());
}
