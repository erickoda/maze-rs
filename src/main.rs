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
    let mut maze = Maze::get_from_file("Labirintos/50x50/exemplo_labirinto.txt".into());
    let a_star_path = search::depth_first::depth_first_search(maze.clone().map);

    if a_star_path.is_none() {
        return;
    }

    let depth_first_path = a_star_path.unwrap();

    for i in 0..maze.clone().map.len() {
        for j in 0..maze.clone().map[i].len() {
            for position in depth_first_path.iter() {
                if i == position.x && j == position.y {
                    maze.map[i][j] = MazeSquareRole::PathToExit;
                }
            }
        }
    }

    for i in 0..maze.clone().map.len() {
        for j in 0..maze.clone().map[i].len() {
            let scale = 50. / maze.size as f32 + 3.;
            let position = Vec3::new(i as f32 * scale, j as f32 * scale, 1.);

            match maze.clone().map[i][j] {
                MazeSquareRole::Empty => {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            material: materials.add(Color::from(WHITE)),
                            transform: {
                                Transform {
                                    translation: position,
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::splat(scale),
                                }
                            },
                            ..Default::default()
                        },
                        MazeSquare,
                    ));
                }
                MazeSquareRole::Wall => {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            material: materials.add(Color::from(BLACK)),
                            transform: {
                                Transform {
                                    translation: position,
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::splat(scale),
                                }
                            },
                            ..Default::default()
                        },
                        MazeSquare,
                    ));
                }
                MazeSquareRole::Entry => {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            material: materials.add(Color::from(BLUE)),
                            transform: {
                                Transform {
                                    translation: position,
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::splat(scale),
                                }
                            },
                            ..Default::default()
                        },
                        MazeSquare,
                    ));
                }
                MazeSquareRole::Exit => {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            material: materials.add(Color::from(RED)),
                            transform: {
                                Transform {
                                    translation: position,
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::splat(scale),
                                }
                            },
                            ..Default::default()
                        },
                        MazeSquare,
                    ));
                }
                MazeSquareRole::PathToExit => {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            material: materials.add(Color::from(PURPLE)),
                            transform: {
                                Transform {
                                    translation: position,
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::splat(scale),
                                }
                            },
                            ..Default::default()
                        },
                        MazeSquare,
                    ));
                }
            }
        }
    }

    commands.spawn(maze);
}

fn spawn_camera(mut commands: Commands) {
    let position = Vec3::new(0 as f32, 0 as f32, 0.);
    commands.spawn(Camera2dBundle::default());
}
