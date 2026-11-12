//! `FlxDropdown` - a button that opens a list of options.

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window as BevyWindow};

use super::{widget::Widget, UiEvent};

#[derive(Component, Debug, Clone, Default)]
pub struct Dropdown {
    pub label: String,
    pub options: Vec<String>,
    pub selected: usize,
    pub open: bool,
}

impl Dropdown {
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Self {
        Self {
            label: label.into(),
            options,
            ..Default::default()
        }
    }
}

/// Marker for an option row inside a dropdown; `dropdown` is its parent.
#[derive(Component, Debug, Clone)]
pub struct DropdownOption {
    pub index: usize,
    pub dropdown: Entity,
}

pub struct DropdownPlugin;

impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dropdown_input_system);
    }
}

fn dropdown_input_system(
    mut dropdowns: Query<(Entity, &Widget, &mut Dropdown)>,
    options: Query<(Entity, &Widget, &DropdownOption)>,
    bevy_windows: Query<&BevyWindow, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(bevy_window) = bevy_windows.get_single() else {
        return;
    };
    let cursor = bevy_window.cursor_position();

    // Toggle the dropdown open/closed when its button is clicked.
    for (_entity, widget, mut dropdown) in dropdowns.iter_mut() {
        if widget.contains(cursor) && mouse.just_pressed(MouseButton::Left) && widget.enabled {
            dropdown.open = !dropdown.open;
        }
    }

    // Select an option row when clicked.
    for (opt_entity, opt_widget, option) in options.iter() {
        if opt_widget.contains(cursor)
            && mouse.just_pressed(MouseButton::Left)
            && opt_widget.enabled
        {
            if let Ok((_, _, mut dropdown)) = dropdowns.get_mut(option.dropdown) {
                dropdown.selected = option.index;
                dropdown.open = false;
                events.send(UiEvent::DropdownSelected(option.dropdown, option.index));
            }
        }
        let _ = opt_entity;
    }
}
