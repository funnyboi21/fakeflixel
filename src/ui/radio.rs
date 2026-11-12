//! `FlxRadioButton` - mutually exclusive radio buttons within a group.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{assets::FakeFlixelUiAssets, widget::Widget, UiEvent};

#[derive(Component, Debug, Clone, Default)]
pub struct RadioButton {
    pub label: String,
    pub group: String,
    pub selected: bool,
}

impl RadioButton {
    pub fn new(label: impl Into<String>, group: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            group: group.into(),
            ..Default::default()
        }
    }
}

pub struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (radio_input_system, radio_visual_system));
    }
}

/// Selects the clicked radio button.
fn radio_input_system(
    mut radios: Query<(Entity, &Widget, &mut RadioButton)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    let mut winner: Option<(String, Entity)> = None;
    for (entity, widget, mut radio) in radios.iter_mut() {
        if widget.contains(cursor) && mouse.just_pressed(MouseButton::Left) && widget.enabled {
            if !radio.selected {
                radio.selected = true;
                winner = Some((radio.group.clone(), entity));
            }
        }
    }
    // Mutual exclusion: deselect every other button in the winner's group.
    if let Some((group, winner_entity)) = winner {
        for (entity, _, mut radio) in radios.iter_mut() {
            if radio.group == group && entity != winner_entity {
                radio.selected = false;
            }
        }
        events.send(UiEvent::RadioToggled(winner_entity));
    }
}

/// Updates the radio/dot sprite based on selection state.
fn radio_visual_system(
    assets: Res<FakeFlixelUiAssets>,
    mut radios: Query<(&RadioButton, &mut Handle<Image>)>,
) {
    for (radio, mut image) in radios.iter_mut() {
        let handle = if radio.selected {
            assets.radio_dot.clone()
        } else {
            assets.radio.clone()
        };
        if *image != handle {
            *image = handle;
        }
    }
}
