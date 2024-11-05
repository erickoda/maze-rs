use bevy::prelude::*;

use crate::{
    components::MazeSquare,
    maze::table_sizes::DefaultMazeTableSizes,
    user_interface::{
        system::ui_visibility::toggle_ui_visibility,
        theme::{PRIMARY_100, PRIMARY_200, PRIMARY_400},
        MainMenu,
    },
};

use super::{remove::remove_maze, spawn::spawn_chosen_maze};

pub fn choose_maze_button_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DefaultMazeTableSizes),
        (Changed<Interaction>, With<Button>),
    >,
    remove_maze_query: Query<Entity, With<MazeSquare>>,
    mut ui_visibility_query: Query<&mut Visibility, With<MainMenu>>,
    mut windows: Query<&Window>,
) {
    for (interaction, mut background_color, maze) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                remove_maze(&mut commands, &remove_maze_query);
                *background_color = PRIMARY_200.into();
            }
            Interaction::Hovered => {
                *background_color = PRIMARY_400.into();
            }
            Interaction::None => {
                *background_color = PRIMARY_100.into();
            }
        };

        match maze {
            DefaultMazeTableSizes::TenPerTen => {
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    spawn_chosen_maze(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        "assets/Labirintos/10x10/exemplo_labirinto.txt".into(),
                        &mut windows,
                    );
                }
            }
            DefaultMazeTableSizes::FiftyPerFifty => {
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    spawn_chosen_maze(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        "assets/Labirintos/50x50/exemplo_labirinto.txt".into(),
                        &mut windows,
                    );
                }
            }
            DefaultMazeTableSizes::OneHundredPerOneHundred => {
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    spawn_chosen_maze(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        "assets/Labirintos/100x100/exemplo_labirinto.txt".into(),
                        &mut windows,
                    );
                }
            }
            DefaultMazeTableSizes::TwoHundredPerTwoHundred => {
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    spawn_chosen_maze(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        "assets/Labirintos/200x200/exemplo_labirinto.txt".into(),
                        &mut windows,
                    );
                }
            }
        }
    }
}
