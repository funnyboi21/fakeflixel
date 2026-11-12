//! Base widget component and shared geometry.

use bevy::prelude::*;

/// Common data carried by every fakeflixel widget.
#[derive(Component, Debug, Clone, Default)]
pub struct Widget {
    /// Logical screen-space bounds of the widget.
    pub bounds: Rect,
    /// Whether the widget is drawn and interactive.
    pub visible: bool,
    /// Whether the widget responds to input.
    pub enabled: bool,
    /// Whether the widget can receive keyboard focus (tab order).
    pub focusable: bool,
    /// Current visual state (0 = up, 1 = over, 2 = down/selected, ...).
    pub state: u8,
}

impl Widget {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            ..Default::default()
        }
    }

    /// True if `point` lies inside the widget bounds and it is interactive.
    pub fn contains(&self, point: Option<Vec2>) -> bool {
        let Some(point) = point else { return false };
        // Bevy screen space: y increases downward, so min.y is top and max.y is bottom.
        self.visible
            && self.enabled
            && point.x >= self.bounds.min.x
            && point.x <= self.bounds.max.x
            && point.y >= self.bounds.min.y
            && point.y <= self.bounds.max.y
    }
}

/// Marker: widget can receive keyboard focus.
#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub struct Focusable;

/// Marker: widget is a container that may hold child widgets.
#[derive(Component, Debug, Clone, Default)]
pub struct WidgetContainer;

/// Marker for a text caption spawned as a child of a widget (button label,
/// checkbox caption, dropdown caption, ...). `owner` is the widget entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Caption {
    pub owner: Entity,
}

/// No-op plugin registering the base widget types.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, _app: &mut App) {}
}
