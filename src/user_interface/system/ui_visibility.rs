use bevy::prelude::*;

use crate::user_interface::entity::MainMenu;

pub fn toggle_ui_visibility_when_press_escape(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<MainMenu>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        toggle_ui_visibility(&mut query);
    }
}

pub fn toggle_ui_visibility(query: &mut Query<&mut Visibility, With<MainMenu>>) {
    for mut visibility in query.iter_mut() {
        *visibility = if *visibility == Visibility::Visible {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
