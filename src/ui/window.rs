//! `FlxWindow` - draggable window with a title bar and optional close button.

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window as BevyWindow};

use super::{widget::Widget, UiEvent};

/// A draggable window.
#[derive(Component, Debug, Clone, Default)]
pub struct Window {
    pub title: String,
    pub draggable: bool,
    pub close_button: bool,
    /// Offset between cursor and window origin when dragging began.
    pub drag_offset: Vec2,
    pub dragging: bool,
}

impl Window {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            draggable: true,
            close_button: true,
            ..Default::default()
        }
    }
}

/// Marker for the window's draggable title bar.
#[derive(Component, Debug, Clone, Default)]
pub struct WindowTitleBar;

/// Marker for a window's close button; `window` is the entity it closes.
#[derive(Component, Debug, Clone)]
pub struct WindowCloseButton {
    pub window: Entity,
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (window_drag_system, window_close_system));
    }
}

/// Drag a window by its title bar.
fn window_drag_system(
    mut ui_windows: Query<(Entity, &mut Window, &mut Transform, &mut Widget)>,
    bevy_windows: Query<&BevyWindow, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(bevy_window) = bevy_windows.get_single() else {
        return;
    };
    let cursor = bevy_window.cursor_position();
    for (_entity, mut window, mut transform, mut widget) in ui_windows.iter_mut() {
        if !widget.enabled || !window.draggable {
            continue;
        }
        let over = widget.contains(cursor);
        if over && mouse.just_pressed(MouseButton::Left) {
            window.dragging = true;
            if let Some(c) = cursor {
                // Widget bounds are screen-space (y down); the transform lives
                // in world space (y up), so flip the cursor's y.
                let world = Vec2::new(c.x, -c.y);
                window.drag_offset = world - transform.translation.truncate();
            }
        }
        if mouse.just_released(MouseButton::Left) {
            window.dragging = false;
        }
        if window.dragging {
            if let Some(c) = cursor {
                let world = Vec2::new(c.x, -c.y);
                let next = (world - window.drag_offset).extend(transform.translation.z);
                let delta = next.truncate() - transform.translation.truncate();
                transform.translation = next;
                // Keep the screen-space hit bounds in sync (world y is flipped).
                widget.bounds.min += Vec2::new(delta.x, -delta.y);
                widget.bounds.max += Vec2::new(delta.x, -delta.y);
            }
        }
    }
}

/// Emit a close event when a close button is clicked.
fn window_close_system(
    buttons: Query<(&Widget, &WindowCloseButton)>,
    bevy_windows: Query<&BevyWindow, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(bevy_window) = bevy_windows.get_single() else {
        return;
    };
    let cursor = bevy_window.cursor_position();
    for (widget, close) in buttons.iter() {
        if widget.contains(cursor) && mouse.just_pressed(MouseButton::Left) && widget.enabled {
            events.send(UiEvent::WindowClosed(close.window));
        }
    }
}
