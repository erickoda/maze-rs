use bevy::prelude::*;

use crate::user_interface::{
    entity::HoverableButton,
    theme::{PRIMARY_100, PRIMARY_200, PRIMARY_400},
};

pub fn button_styled_when_mouse_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, (With<Button>, With<HoverableButton>)),
    >,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRIMARY_200.into();
            }
            Interaction::Hovered => {
                *background_color = PRIMARY_400.into();
            }
            Interaction::None => {
                *background_color = PRIMARY_100.into();
            }
        };
    }
}
