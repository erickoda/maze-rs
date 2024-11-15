use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let speed = 2.;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += speed;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= speed;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }

        if keyboard_input.pressed(KeyCode::Equal) {
            transform.scale *= 0.97;
        }

        if keyboard_input.pressed(KeyCode::Minus) {
            transform.scale *= 1.03;
        }
    }
}
