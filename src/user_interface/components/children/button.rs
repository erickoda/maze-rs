use super::text::NestedTextBuilder;
use crate::user_interface::theme::{DARK_GREY, PRIMARY_100};
use bevy::prelude::*;

#[derive(Default)]
pub struct NestedButtonBuilder {
    pub font: Option<Handle<Font>>,
    pub text: Option<String>,
}

impl NestedButtonBuilder {
    pub fn add_font(&mut self, font: Handle<Font>) -> &mut Self {
        self.font = Some(font);
        self
    }

    pub fn add_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = Some(text.into());
        self
    }

    pub fn spawn(&mut self, builder: &mut ChildBuilder, bundle: impl Bundle) {
        let text = self.text.clone().unwrap_or_default();
        let font = self.font.clone().unwrap_or_default();

        builder
            .spawn(ButtonBundle {
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
            })
            .insert(bundle)
            .with_children(|builder| {
                NestedTextBuilder::default()
                    .set_color(DARK_GREY)
                    .set_font_size(12.)
                    .set_font(font.clone())
                    .set_text(text)
                    .spawn(builder);
            });
    }
}
