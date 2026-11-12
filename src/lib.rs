//! FakeFlixel engine layer: a HaxeFlixel (`flixel`/`flixel-ui`) inspired UI
//! framework built on Bevy.
//!
//! The engine-facing facade for FakeFlixel UI code. Import from this crate
//! root (or its submodules) rather than reaching into `fakeflixel::ui::*`:
//!
//! ```ignore
//! use fakeflixel::{spawn_button, spawn_label, FakeFlixelUiAssets};
//! ```
//!
//! Layout:
//! - [`ui`]    — widget components, per-widget systems and the UI plugin
//! - [`theme`] — [`UiTheme`] resource with shared sizes/colors
//! - [`coords`]— top-left-origin screen-space → Bevy world-space helpers

pub mod coords;
pub mod theme;
pub mod ui;

use bevy::prelude::*;

pub use coords::{
    centered_x, centered_y, screen_center, screen_rect, screen_to_world, screen_to_world2,
};
pub use theme::UiTheme;
pub use ui::assets::FakeFlixelUiAssets;
pub use ui::spawn::{
    spawn_button, spawn_checkbox, spawn_dropdown, spawn_dropdown_option, spawn_label, spawn_panel,
    spawn_radio_button, spawn_text_input, spawn_window,
};
pub use ui::{FakeFlixelUiPlugin, UiEvent, UiFocus};

/// Grouped widget component types, namespaced to avoid clashing with Bevy's
/// own `Button`/`Window`/`Panel` UI components.
pub mod widgets {
    pub use crate::ui::button::{Button, ButtonLabel};
    pub use crate::ui::checkbox::CheckBox;
    pub use crate::ui::dropdown::{Dropdown, DropdownOption};
    pub use crate::ui::label::{Label, LabelText};
    pub use crate::ui::panel::{Panel, PanelCloseButton, PanelTitleBar};
    pub use crate::ui::radio::RadioButton;
    pub use crate::ui::text_input::TextInput;
    pub use crate::ui::widget::{Focusable, Widget, WidgetContainer};
    pub use crate::ui::window::{Window as FlxWindow, WindowCloseButton, WindowTitleBar};
}

/// Engine-facing fakeflixel plugin: registers the UI framework (theme,
/// widget assets, input plumbing) with the Bevy app.
///
/// `EngineCorePlugin` adds this once; individual widget plugins are wired up
/// inside [`FakeFlixelUiPlugin`].
pub struct FakeFlixelPlugin;

impl Plugin for FakeFlixelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
            .add_plugins(ui::FakeFlixelUiPlugin);
    }
}
