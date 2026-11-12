//! `FlxButton`-style clickable button with up/over/down frames.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{assets::FakeFlixelUiAssets, widget::Widget, UiEvent};

/// A clickable button.
#[derive(Component, Debug, Clone, Default)]
pub struct Button {
    pub label: String,
    pub hovered: bool,
    pub pressed: bool,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }
}

/// Marker for the text child of a button.
#[derive(Component, Debug, Clone, Default)]
pub struct ButtonLabel;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_input_system, button_visual_system));
    }
}

/// Reads input and drives button pressed/hovered state.
fn button_input_system(
    mut buttons: Query<(Entity, &Widget, &mut Button)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    for (entity, widget, mut button) in buttons.iter_mut() {
        let hovered = widget.contains(cursor);
        button.hovered = hovered && widget.enabled;

        if button.hovered && mouse.just_pressed(MouseButton::Left) {
            button.pressed = true;
        }
        if mouse.just_released(MouseButton::Left) {
            if button.pressed && button.hovered {
                events.send(UiEvent::ButtonPressed(entity));
            }
            button.pressed = false;
        }
    }
}

/// Swaps the button texture based on its current state.
fn button_visual_system(
    assets: Res<FakeFlixelUiAssets>,
    mut buttons: Query<(&Widget, &Button, &mut Handle<Image>)>,
) {
    for (widget, button, mut image) in buttons.iter_mut() {
        let handle = if !widget.enabled {
            assets.button_thin.clone()
        } else if button.pressed {
            assets.button_toggle.clone()
        } else if button.hovered {
            assets.button.clone()
        } else {
            assets.button_thin.clone()
        };
        if *image != handle {
            *image = handle;
        }
    }
}
