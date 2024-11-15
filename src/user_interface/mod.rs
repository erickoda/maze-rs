pub mod components;
mod entity;
pub mod system;
pub mod theme;

use crate::{
    maze::table_sizes::DefaultMazeTableSizes, search::systems::recolor::RecolorSpeedChanger,
};
use bevy::prelude::*;
use components::{
    children::{
        button::NestedButtonBuilder, flex_layout::ChildrenFlexLayout, text::NestedTextBuilder,
    },
    parent::flex_layout::ParentFlexLayout,
};
use entity::{HoverableButton, MainMenu, SpeedBar};
use theme::{COMPLEMENTARY_100, PRIMARY_100};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> =
        asset_server.load("fonts/JetBrainsMono/JetBrainsMonoNerdFont-Regular.ttf");
    let bold_font: Handle<Font> =
        asset_server.load("fonts/JetBrainsMono/JetBrainsMonoNerdFont-ExtraBold.ttf");

    ParentFlexLayout::spawn_vertical(&mut commands)
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
            ChildrenFlexLayout::spawn_horizontal(builder).with_children(|builder| {
                NestedButtonBuilder::default()
                    .add_font(font.clone())
                    .add_text(DefaultMazeTableSizes::TenPerTen)
                    .spawn(builder, (HoverableButton, DefaultMazeTableSizes::TenPerTen));

                NestedButtonBuilder::default()
                    .add_font(font.clone())
                    .add_text(DefaultMazeTableSizes::FiftyPerFifty)
                    .spawn(
                        builder,
                        (HoverableButton, DefaultMazeTableSizes::FiftyPerFifty),
                    );

                NestedButtonBuilder::default()
                    .add_font(font.clone())
                    .add_text(DefaultMazeTableSizes::OneHundredPerOneHundred)
                    .spawn(
                        builder,
                        (
                            HoverableButton,
                            DefaultMazeTableSizes::OneHundredPerOneHundred,
                        ),
                    );

                NestedButtonBuilder::default()
                    .add_font(font.clone())
                    .add_text(DefaultMazeTableSizes::TwoHundredPerTwoHundred)
                    .spawn(
                        builder,
                        (
                            HoverableButton,
                            DefaultMazeTableSizes::TwoHundredPerTwoHundred,
                        ),
                    );
            });
        })
        .with_children(|builder| {
            ChildrenFlexLayout::spawn_vertical(builder)
                .with_children(|builder| {
                    ChildrenFlexLayout::spawn_horizontal(builder).with_children(|builder| {
                        NestedButtonBuilder::default()
                            .add_font(font.clone())
                            .add_text("󰳡 Speed Up")
                            .spawn(builder, (HoverableButton, RecolorSpeedChanger::Faster));

                        NestedButtonBuilder::default()
                            .add_font(font.clone())
                            .add_text("󰳛 Speed Down")
                            .spawn(builder, (HoverableButton, RecolorSpeedChanger::Slower));
                    });
                })
                .with_children(|builder| {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                height: Val::Px(4.),
                                border: UiRect::all(Val::Px(1.)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(100.),
                                        height: Val::Px(4.),
                                        border: UiRect::all(Val::Px(0.3)),
                                        ..default()
                                    },
                                    background_color: COMPLEMENTARY_100.into(),
                                    ..default()
                                },
                                SpeedBar,
                            ));
                        });
                });
        });
}
