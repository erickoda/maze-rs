use bevy::prelude::*;

use crate::{
    search::systems::recolor::{
        components::maze_animation_speed::MazeAnimationSpeed,
        resources::maze_animation_speed_changer::MazeAnimationSpeedChanger,
    },
    user_interface::entity::SpeedBar,
};

pub fn change_speed_bar_length(
    mut interaction_query: Query<&mut Style, (With<Node>, With<SpeedBar>)>,
    maze_animation_speed: Res<MazeAnimationSpeed>,
) {
    for mut style in interaction_query.iter_mut() {
        style.width = Val::Percent(
            u32::from(maze_animation_speed.clone()) as f32 / MazeAnimationSpeed::get_max() as f32
                * 100.,
        );
    }
}

pub fn change_speed_of_search_animation(
    interaction_query: Query<
        (&Interaction, &MazeAnimationSpeedChanger),
        (Changed<Interaction>, With<Button>),
    >,
    mut animation_speed: ResMut<MazeAnimationSpeed>,
) {
    for (interaction, speed_changer) in &mut interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match speed_changer {
            MazeAnimationSpeedChanger::Slower => {
                animation_speed.slower();
            }
            MazeAnimationSpeedChanger::Faster => {
                animation_speed.faster();
            }
        }
    }
}
