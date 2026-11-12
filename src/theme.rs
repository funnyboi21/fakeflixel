//! Shared fakeflixel UI theme: default sizes, colors and spacing.
//!
//! Mirrors the look-and-feel constants `flixel-ui` bakes into its widgets so
//! game code can restyle the whole UI from one resource.

use bevy::prelude::*;

/// Global UI theme. Systems and spawn helpers read this for consistent
/// typography, widget sizing and accent colors.
#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    /// Default text color for labels.
    pub text_color: Color,
    /// Default label font size (px).
    pub text_size: f32,
    /// Font size used for widget captions (buttons, checkboxes, ...).
    pub widget_text_size: f32,
    /// Standard spacing between stacked widgets.
    pub spacing: f32,
    /// Inner padding of panels/windows.
    pub padding: f32,
    /// Height of a window/panel title bar.
    pub title_height: f32,
    /// Edge length of checkboxes and radio buttons.
    pub indicator_size: f32,
    /// Default button size.
    pub button_size: Vec2,
    /// Default text-input height.
    pub input_height: f32,
    /// Default dropdown height.
    pub dropdown_height: f32,
    /// Color applied to disabled widgets.
    pub disabled_color: Color,
    /// Color applied while hovering (used by widgets that tint on hover).
    pub hover_tint: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            text_color: Color::WHITE,
            text_size: 24.0,
            widget_text_size: 20.0,
            spacing: 8.0,
            padding: 8.0,
            title_height: 28.0,
            indicator_size: 24.0,
            button_size: Vec2::new(160.0, 32.0),
            input_height: 32.0,
            dropdown_height: 32.0,
            disabled_color: Color::srgba(1.0, 1.0, 1.0, 0.4),
            hover_tint: Color::srgba(1.0, 1.0, 1.0, 0.15),
        }
    }
}
