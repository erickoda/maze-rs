mod camera;
mod maze;
mod search;
mod user_interface;

use bevy::prelude::*;
use camera::CameraPlugin;
use search::SearchPlugin;
use user_interface::system::UserInterfacePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SearchPlugin)
        .add_plugins(UserInterfacePlugin)
        .run();
}
