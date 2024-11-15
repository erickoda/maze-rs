use bevy::prelude::*;

use crate::{
    maze::{board::sizes::DefaultMazeBoardSizes, board::MazeBoard},
    search::systems::maze_board_tasks::{
        send::send_maze_board_background_task, MazeBoardTasks, MazeBoardTasksController,
    },
    user_interface::{entity::MainMenu, system::ui_visibility::toggle_ui_visibility},
};

pub fn choose_maze_button_system(
    mut interaction_query: Query<
        (&Interaction, &DefaultMazeBoardSizes),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_visibility_query: Query<&mut Visibility, With<MainMenu>>,
    maze_tasks_channel: Res<MazeBoardTasksController>,
) {
    let mut maze_board;

    for (interaction, maze) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        send_maze_board_background_task(&maze_tasks_channel, MazeBoardTasks::Destroy());

        maze_board = match maze {
            DefaultMazeBoardSizes::TenPerTen => {
                MazeBoard::get_from_file("assets/Labirintos/10x10/exemplo_labirinto.txt")
            }
            DefaultMazeBoardSizes::FiftyPerFifty => {
                MazeBoard::get_from_file("assets/Labirintos/50x50/exemplo_labirinto.txt")
            }
            DefaultMazeBoardSizes::OneHundredPerOneHundred => {
                MazeBoard::get_from_file("assets/Labirintos/100x100/exemplo_labirinto.txt")
            }
            DefaultMazeBoardSizes::TwoHundredPerTwoHundred => {
                MazeBoard::get_from_file("assets/Labirintos/200x200/exemplo_labirinto.txt")
            }
        };

        send_maze_board_background_task(&maze_tasks_channel, MazeBoardTasks::Create(maze_board));
        toggle_ui_visibility(&mut ui_visibility_query);
    }
}
