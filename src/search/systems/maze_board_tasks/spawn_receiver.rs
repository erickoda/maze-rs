use super::{receiver::board_background_tasks_receiver, MazeBoardTasksController};
use bevy::{prelude::*, tasks::IoTaskPool};

pub fn spawn_maze_board_tasks_receiver(mut commands: Commands) {
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_update, rx_update) = async_channel::unbounded();

    IoTaskPool::get()
        .spawn(async move {
            board_background_tasks_receiver(rx_control, tx_update).await;
        })
        .detach();

    commands.insert_resource(MazeBoardTasksController {
        tx_control,
        rx_update,
    });
}
