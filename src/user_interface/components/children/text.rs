use bevy::prelude::*;

use crate::user_interface::theme::PRIMARY_100;

#[derive(Default)]
pub struct NestedTextBuilder {
    pub font: Option<Handle<Font>>,
    pub text: Option<String>,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
}

impl NestedTextBuilder {
    pub fn set_font(&mut self, font: Handle<Font>) -> &mut NestedTextBuilder {
        self.font = Some(font);
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) -> &mut NestedTextBuilder {
        self.text = Some(text.into());
        self
    }

    pub fn set_font_size(&mut self, font_size: f32) -> &mut NestedTextBuilder {
        self.font_size = Some(font_size);
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut NestedTextBuilder {
        self.color = Some(color);
        self
    }

    pub fn spawn(&self, builder: &mut ChildBuilder) {
        let text = self.text.clone().unwrap_or_default();
        let font = self.font.clone().unwrap_or_default();
        let font_size = self.font_size.unwrap_or(16.);
        let color = self.color.unwrap_or(PRIMARY_100);

        builder.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size,
                color,
            },
        ));
    }
}
