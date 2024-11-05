use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn move_camera(
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
