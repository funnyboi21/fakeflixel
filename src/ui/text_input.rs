//! `FlxTextInput` - single-line keyboard text entry.

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::widget::Widget;
use super::{UiEvent, UiFocus};

#[derive(Component, Debug, Clone, Default)]
pub struct TextInput {
    pub text: String,
    pub cursor: usize,
    pub focused: bool,
    pub max_length: Option<usize>,
    pub password: bool,
}

impl TextInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
}

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (text_input_focus, text_input_typing));
    }
}

/// Click a field to focus it; click elsewhere to blur.
fn text_input_focus(
    mut inputs: Query<(Entity, &Widget, &mut TextInput)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut focus: ResMut<UiFocus>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let mut hit: Option<Entity> = None;
    for (entity, widget, mut input) in inputs.iter_mut() {
        if widget.contains(cursor) && widget.enabled {
            input.focused = true;
            hit = Some(entity);
        } else {
            input.focused = false;
        }
    }
    focus.focused = hit;
}

/// Collect keyboard input into the focused text field.
fn text_input_typing(
    focus: Res<UiFocus>,
    mut inputs: Query<(Entity, &mut TextInput)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut chars: EventReader<KeyboardInput>,
    mut events: EventWriter<UiEvent>,
) {
    let Some(focused_entity) = focus.focused else {
        return;
    };
    let Ok((_entity, mut input)) = inputs.get_mut(focused_entity) else {
        return;
    };
    if !input.focused {
        return;
    }

    let max = input.max_length.unwrap_or(usize::MAX);
    for c in chars.read() {
        if c.state != ButtonState::Pressed {
            continue;
        }
        let Key::Character(text) = &c.logical_key else {
            continue;
        };
        for ch in text.chars() {
            if input.text.len() >= max {
                break;
            }
            let cursor = input.cursor;
            input.text.insert(cursor, ch);
            input.cursor = cursor + ch.len_utf8();
        }
    }

    if keys.just_pressed(KeyCode::Backspace) && input.cursor > 0 {
        let cursor = input.cursor - 1;
        input.cursor = cursor;
        input.text.remove(cursor);
    }
    if keys.just_pressed(KeyCode::Delete) && input.cursor < input.text.len() {
        let cursor = input.cursor;
        input.text.remove(cursor);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        input.cursor = input.cursor.saturating_sub(1);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        input.cursor = (input.cursor + 1).min(input.text.len());
    }
    if keys.just_pressed(KeyCode::Enter) {
        events.send(UiEvent::TextSubmitted(focused_entity, input.text.clone()));
    }
}
