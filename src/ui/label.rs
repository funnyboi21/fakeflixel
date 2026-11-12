//! `FlxText`-style label: text with an optional background texture.

use bevy::prelude::*;

/// A text label.
#[derive(Component, Debug, Clone, Default)]
pub struct Label {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub centered: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 24.0,
            color: Color::WHITE,
            centered: false,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }
}

/// Marker for the text child of a label.
#[derive(Component, Debug, Clone, Default)]
pub struct LabelText;

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, _app: &mut App) {}
}
