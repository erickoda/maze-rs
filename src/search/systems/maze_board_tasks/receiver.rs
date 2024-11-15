use super::MazeBoardTasks;
use async_channel::{Receiver, Sender};

pub async fn board_background_tasks_receiver(
    rx_control: Receiver<MazeBoardTasks>,
    tx_update: Sender<MazeBoardTasks>,
) {
    while let Ok(recolor) = rx_control.recv().await {
        match recolor {
            MazeBoardTasks::Destroy() => {
                tx_update
                    .send(MazeBoardTasks::Destroy())
                    .await
                    .expect("Error sending updates over channel");
            }
            MazeBoardTasks::Create(maze_path) => {
                tx_update
                    .send(MazeBoardTasks::Create(maze_path))
                    .await
                    .expect("Error sending updates over channel");
            }
            MazeBoardTasks::Update(path_with_color) => {
                tx_update
                    .send(MazeBoardTasks::Update(path_with_color))
                    .await
                    .expect("Error sending updates over channel");
            }
            MazeBoardTasks::Clear() => {
                tx_update
                    .send(MazeBoardTasks::Clear())
                    .await
                    .expect("Error sending updates over channel");
            }
        }
    }
}
