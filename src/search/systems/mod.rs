pub mod animations;
pub mod recolor;
mod remove;
mod spawn;

use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};
use recolor::PendingColorUpdates;
use remove::remove_maze;
use spawn::spawn_chosen_maze;

use crate::maze::{MazeSquare, MazeTable, Position};

#[derive(Debug)]
pub enum MazeTableTasks {
    Create(MazeTable),
    Destroy(),
    Update(Color, Position),
}

#[derive(Resource)]
pub struct MazeTableTasksController {
    pub tx_control: Sender<MazeTableTasks>,
    pub rx_update: Receiver<MazeTableTasks>,
}

pub fn spawn_maze_table_tasks_receiver(mut commands: Commands) {
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_update, rx_update) = async_channel::unbounded();

    IoTaskPool::get()
        .spawn(async move {
            table_background_tasks_receiver(rx_control, tx_update).await;
        })
        .detach();

    commands.insert_resource(MazeTableTasksController {
        tx_control,
        rx_update,
    });
}

pub fn execute_maze_table_tasks(
    maze_tasks_channel: Res<MazeTableTasksController>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&Window>,
    mut pending_updates: ResMut<PendingColorUpdates>,
    destroy_query: Query<Entity, With<MazeSquare>>,
) {
    while let Ok(msg) = maze_tasks_channel.rx_update.try_recv() {
        match msg {
            MazeTableTasks::Create(maze_table) => {
                spawn_chosen_maze(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    maze_table.clone(),
                    &mut windows,
                );
            }
            MazeTableTasks::Update(color, position) => {
                println!("Received update: {:?} at {:?}", color, position);
                pending_updates.updates.push_back((color, position));
            }
            MazeTableTasks::Destroy() => {
                println!("Destroying the task");
                remove_maze(&mut commands, &destroy_query);
            }
        }
    }
}

pub fn send_maze_table_background_task(
    maze_tasks_channel: &Res<MazeTableTasksController>,
    recolor_message: MazeTableTasks,
) {
    if let Err(e) = maze_tasks_channel.tx_control.try_send(recolor_message) {
        println!("Failed to send message to the net task: {:?}", e);
    }
}

async fn table_background_tasks_receiver(
    rx_control: Receiver<MazeTableTasks>,
    tx_update: Sender<MazeTableTasks>,
) {
    while let Ok(recolor) = rx_control.recv().await {
        println!("Net task received a message");
        match recolor {
            MazeTableTasks::Destroy() => {
                println!("Destroying the task");
                tx_update
                    .send(MazeTableTasks::Destroy())
                    .await
                    .expect("Error sending updates over channel");
            }
            MazeTableTasks::Create(maze_path) => {
                tx_update
                    .send(MazeTableTasks::Create(maze_path))
                    .await
                    .expect("Error sending updates over channel");
            }
            MazeTableTasks::Update(color, position) => {
                tx_update
                    .send(MazeTableTasks::Update(color, position))
                    .await
                    .expect("Error sending updates over channel");
            }
        }
    }
}
