//! `FlxWindow`-style panel: a draggable container with a background texture.
//! Used for editor inspectors, property sheets, and dialog boxes.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::widget::Widget;
use super::UiEvent;

/// A panel with a textured background and optional title bar.
#[derive(Component, Debug, Clone, Default)]
pub struct Panel {
    pub title: String,
    pub draggable: bool,
    pub close_button: bool,
    pub drag_offset: Vec2,
    pub dragging: bool,
}

impl Panel {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            draggable: true,
            close_button: false,
            ..Default::default()
        }
    }

    pub fn with_draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    pub fn with_close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }
}

/// Marker for a panel's draggable title bar.
#[derive(Component, Debug, Clone, Default)]
pub struct PanelTitleBar;

/// Marker for a panel's close button.
#[derive(Component, Debug, Clone)]
pub struct PanelCloseButton {
    pub panel: Entity,
}

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (panel_drag_system, panel_close_system));
    }
}

/// Drag a panel by its title bar.
fn panel_drag_system(
    mut panels: Query<(Entity, &mut Panel, &mut Transform, &mut Widget)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    for (_entity, mut panel, mut transform, mut widget) in panels.iter_mut() {
        if !widget.enabled || !panel.draggable {
            continue;
        }
        let over = widget.contains(cursor);
        if over && mouse.just_pressed(MouseButton::Left) {
            panel.dragging = true;
            if let Some(c) = cursor {
                // Widget bounds are screen-space (y down); the transform lives
                // in world space (y up), so flip the cursor's y.
                let world = Vec2::new(c.x, -c.y);
                panel.drag_offset = world - transform.translation.truncate();
            }
        }
        if mouse.just_released(MouseButton::Left) {
            panel.dragging = false;
        }
        if panel.dragging {
            if let Some(c) = cursor {
                let world = Vec2::new(c.x, -c.y);
                let next = (world - panel.drag_offset).extend(transform.translation.z);
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
fn panel_close_system(
    buttons: Query<(&Widget, &PanelCloseButton)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<UiEvent>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();
    for (widget, close) in buttons.iter() {
        if widget.contains(cursor) && mouse.just_pressed(MouseButton::Left) && widget.enabled {
            events.send(UiEvent::WindowClosed(close.panel));
        }
    }
}
