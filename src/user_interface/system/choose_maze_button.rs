use bevy::prelude::*;

use crate::{
    maze::{table_sizes::DefaultMazeTableSizes, MazeTable},
    search::systems::{send_maze_table_background_task, MazeTableTasks, MazeTableTasksController},
    user_interface::{entity::MainMenu, system::ui_visibility::toggle_ui_visibility},
};

pub fn choose_maze_button_system(
    mut interaction_query: Query<
        (&Interaction, &DefaultMazeTableSizes),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_visibility_query: Query<&mut Visibility, With<MainMenu>>,
    maze_tasks_channel: Res<MazeTableTasksController>,
) {
    let mut maze_table;

    for (interaction, maze) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        send_maze_table_background_task(&maze_tasks_channel, MazeTableTasks::Destroy());

        maze_table = match maze {
            DefaultMazeTableSizes::TenPerTen => {
                MazeTable::get_from_file("assets/Labirintos/10x10/exemplo_labirinto.txt")
            }
            DefaultMazeTableSizes::FiftyPerFifty => {
                MazeTable::get_from_file("assets/Labirintos/50x50/exemplo_labirinto.txt")
            }
            DefaultMazeTableSizes::OneHundredPerOneHundred => {
                MazeTable::get_from_file("assets/Labirintos/100x100/exemplo_labirinto.txt")
            }
            DefaultMazeTableSizes::TwoHundredPerTwoHundred => {
                MazeTable::get_from_file("assets/Labirintos/200x200/exemplo_labirinto.txt")
            }
        };

        send_maze_table_background_task(&maze_tasks_channel, MazeTableTasks::Create(maze_table));
        toggle_ui_visibility(&mut ui_visibility_query);
    }
}
