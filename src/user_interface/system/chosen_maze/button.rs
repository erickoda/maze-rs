use bevy::prelude::*;

use crate::{
    maze::{table_sizes::DefaultMazeTableSizes, MazeTable},
    search::systems::{send_maze_table_background_task, MazeTableTasks, MazeTableTasksController},
    user_interface::{
        system::ui_visibility::toggle_ui_visibility,
        theme::{PRIMARY_100, PRIMARY_200, PRIMARY_400},
        MainMenu,
    },
};

pub fn choose_maze_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DefaultMazeTableSizes),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_visibility_query: Query<&mut Visibility, With<MainMenu>>,
    maze_tasks_channel: Res<MazeTableTasksController>,
) {
    for (interaction, mut background_color, maze) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                send_maze_table_background_task(&maze_tasks_channel, MazeTableTasks::Destroy());
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
                let maze_table = MazeTable::get_from_file(
                    "assets/Labirintos/10x10/exemplo_labirinto.txt".into(),
                );
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    send_maze_table_background_task(
                        &maze_tasks_channel,
                        MazeTableTasks::Create(maze_table.clone()),
                    );
                }
            }
            DefaultMazeTableSizes::FiftyPerFifty => {
                let maze_table = MazeTable::get_from_file(
                    "assets/Labirintos/50x50/exemplo_labirinto.txt".into(),
                );
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    send_maze_table_background_task(
                        &maze_tasks_channel,
                        MazeTableTasks::Create(maze_table.clone()),
                    );
                }
            }
            DefaultMazeTableSizes::OneHundredPerOneHundred => {
                let maze_table = MazeTable::get_from_file(
                    "assets/Labirintos/100x100/exemplo_labirinto.txt".into(),
                );
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    send_maze_table_background_task(
                        &maze_tasks_channel,
                        MazeTableTasks::Create(maze_table.clone()),
                    );
                }
            }
            DefaultMazeTableSizes::TwoHundredPerTwoHundred => {
                let maze_table = MazeTable::get_from_file(
                    "assets/Labirintos/200x200/exemplo_labirinto.txt".into(),
                );
                if *interaction == Interaction::Pressed {
                    toggle_ui_visibility(&mut ui_visibility_query);
                    send_maze_table_background_task(
                        &maze_tasks_channel,
                        MazeTableTasks::Create(maze_table.clone()),
                    );
                }
            }
        }
    }
}
