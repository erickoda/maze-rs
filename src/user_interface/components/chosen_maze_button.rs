use bevy::prelude::*;

use crate::{
    maze::table_sizes::DefaultMazeTableSizes,
    user_interface::theme::{DARK_GREY, PRIMARY_100},
};

use super::text::NestedTextBuilder;

pub fn spawn_choose_maze_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    chosen_maze: DefaultMazeTableSizes,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(8.)),
                background_color: PRIMARY_100.into(),
                ..default()
            },
            chosen_maze.clone(),
        ))
        .with_children(|builder| {
            NestedTextBuilder::default()
                .set_color(DARK_GREY)
                .set_font_size(12.)
                .set_font(font.clone())
                .set_text(chosen_maze)
                .spawn(builder);
        });
}
