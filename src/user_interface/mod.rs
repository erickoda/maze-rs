pub mod components;
pub mod system;
pub mod theme;

use bevy::prelude::*;
use components::{chosen_maze_button::spawn_choose_maze_button, text::NestedTextBuilder};
use theme::{DARK_GREY, PRIMARY_100};

use crate::maze::table_sizes::DefaultMazeTableSizes;

#[derive(Component)]
pub struct MainMenu;

pub fn spawn_user_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Poppins/Poppins-Regular.ttf");
    let bold_font: Handle<Font> = asset_server.load("fonts/Poppins/Poppins-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(DARK_GREY),
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|builder| {
            NestedTextBuilder::default()
                .set_font(bold_font.clone())
                .set_text("The Maze")
                .set_color(PRIMARY_100)
                .set_font_size(32.)
                .spawn(builder);
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_choose_maze_button(
                        builder,
                        font.clone(),
                        DefaultMazeTableSizes::TenPerTen,
                    );
                    spawn_choose_maze_button(
                        builder,
                        font.clone(),
                        DefaultMazeTableSizes::FiftyPerFifty,
                    );
                    spawn_choose_maze_button(
                        builder,
                        font.clone(),
                        DefaultMazeTableSizes::OneHundredPerOneHundred,
                    );
                    spawn_choose_maze_button(
                        builder,
                        font.clone(),
                        DefaultMazeTableSizes::TwoHundredPerTwoHundred,
                    );
                });
        });
}
