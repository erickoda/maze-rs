use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::user_interface::theme::background_color::BACKGROUND_COLOR;

pub struct ParentFlexLayout;

impl ParentFlexLayout {
    pub fn spawn_vertical<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(BACKGROUND_COLOR),
            ..Default::default()
        })
    }

    pub fn spawn_horizontal<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(BACKGROUND_COLOR),
            ..Default::default()
        })
    }
}
