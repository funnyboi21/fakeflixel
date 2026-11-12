//! UI widgets mirroring HaxeFlixel's `flixel-ui`:
//! buttons, check boxes, radio buttons, text inputs, windows and dropdowns,
//! with a shared focus manager and a placeholder-texture asset cache.

pub mod assets;
pub mod button;
pub mod checkbox;
pub mod dropdown;
pub mod label;
pub mod panel;
pub mod radio;
pub mod spawn;
pub mod text_input;
pub mod widget;
pub mod window;

use bevy::prelude::*;

use self::{
    assets::load_fakeflixel_ui_assets, button::ButtonPlugin, checkbox::CheckBoxPlugin,
    dropdown::DropdownPlugin, label::LabelPlugin, panel::PanelPlugin, radio::RadioPlugin,
    text_input::TextInputPlugin, widget::WidgetPlugin, window::WindowPlugin,
};

/// Shared UI focus state: which widget currently has keyboard focus.
#[derive(Resource, Default, Debug, Clone)]
pub struct UiFocus {
    pub focused: Option<Entity>,
}

/// Events emitted by fakeflixel widgets so game code can react to input.
#[derive(Event, Debug, Clone)]
pub enum UiEvent {
    ButtonPressed(Entity),
    CheckBoxToggled(Entity, bool),
    RadioToggled(Entity),
    TextSubmitted(Entity, String),
    DropdownSelected(Entity, usize),
    WindowClosed(Entity),
}

pub struct FakeFlixelUiPlugin;

impl Plugin for FakeFlixelUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_event::<UiEvent>()
            .add_systems(Startup, load_fakeflixel_ui_assets)
            .add_plugins((
                WidgetPlugin,
                ButtonPlugin,
                CheckBoxPlugin,
                RadioPlugin,
                DropdownPlugin,
                LabelPlugin,
                PanelPlugin,
                TextInputPlugin,
                WindowPlugin,
            ));
    }
}
