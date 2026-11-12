//! `FlxCheckBox` - a toggleable check box with a label.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{assets::FakeFlixelUiAssets, widget::Widget, UiEvent};

#[derive(Component, Debug, Clone, Default)]
pub struct CheckBox {
    pub label: String,
    pub checked: bool,
}

impl CheckBox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }
}

pub struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, checkbox_input_system);
    }
}

fn checkbox_input_system(
    mut boxes: Query<(Entity, &Widget, &mut CheckBox, &mut Handle<Image>)>,
    assets: Res<FakeFlixelUiAssets>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    for (entity, widget, mut box_, mut image) in boxes.iter_mut() {
        if widget.contains(cursor) && mouse.just_pressed(MouseButton::Left) && widget.enabled {
            box_.checked = !box_.checked;
            events.send(UiEvent::CheckBoxToggled(entity, box_.checked));
        }
        let handle = if box_.checked {
            assets.check_mark.clone()
        } else {
            assets.check_box.clone()
        };
        if *image != handle {
            *image = handle;
        }
    }
}
