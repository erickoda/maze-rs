use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ChildrenFlexLayout;

impl ChildrenFlexLayout {
    pub fn spawn_vertical<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
        builder.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
    }

    pub fn spawn_horizontal<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
        builder.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
    }
}
