//! FakeFlixel is a Flixel-inspired 2D game framework with a framework-owned,
//! headless-testable runtime. Its core has no third-party dependencies.
//!
//! The previous Bevy UI implementation is available as the optional
//! `bevy-ui` compatibility feature while widgets are migrated.

pub mod core;
pub mod widgets;

#[cfg(feature = "minifb")]
mod minifb_backend;

#[cfg(feature = "bevy-ui")]
pub mod coords;
#[cfg(feature = "bevy-ui")]
pub mod theme;
#[cfg(feature = "bevy-ui")]
pub mod ui;

#[cfg(feature = "bevy-ui")]
pub use coords::{
    centered_x, centered_y, screen_center, screen_rect, screen_to_world, screen_to_world2,
};
#[cfg(feature = "bevy-ui")]
pub use theme::UiTheme;
#[cfg(feature = "bevy-ui")]
pub use ui::assets::FakeFlixelUiAssets;
#[cfg(feature = "bevy-ui")]
pub use ui::spawn::{
    spawn_button, spawn_checkbox, spawn_dropdown, spawn_dropdown_option, spawn_label, spawn_panel,
    spawn_radio_button, spawn_text_input, spawn_window,
};
#[cfg(feature = "bevy-ui")]
pub use ui::{FakeFlixelUiPlugin, UiEvent, UiFocus};

pub use core::{
    AssetError, AssetId, AssetStore, AudioBackend, Backend, Camera, Color, FrameBuffer,
    FrameBufferError, Game, GameConfig, GameContext, GameError, Group, ImageAsset, InputSnapshot,
    KeyCode, MouseButton, Rect, Renderer, SoftwareRenderer, Sprite, State, Vec2, SCREEN_SIZE,
};
#[cfg(feature = "minifb")]
pub use minifb_backend::MinifbBackend;

#[cfg(feature = "bevy-ui")]
pub struct FakeFlixelPlugin;

#[cfg(feature = "bevy-ui")]
impl bevy::prelude::Plugin for FakeFlixelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UiTheme>()
            .add_plugins(ui::FakeFlixelUiPlugin);
    }
}
