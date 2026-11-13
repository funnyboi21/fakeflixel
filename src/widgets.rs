//! Framework-owned UI identity, state, and pointer handling.
//!
//! The initial native UI layer covers buttons and check boxes. The complete
//! Bevy widget set remains available through the optional `bevy-ui` feature
//! while the remaining controls are migrated.

use crate::core::{InputSnapshot, MouseButton, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    pub id: WidgetId,
    pub bounds: Rect,
    pub visible: bool,
    pub enabled: bool,
}

impl Widget {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self {
            id,
            bounds,
            visible: true,
            enabled: true,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.visible && self.enabled && self.bounds.contains(crate::core::Vec2::new(x, y))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub widget: Widget,
    pub label: String,
    pub hovered: bool,
    pub pressed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBox {
    pub widget: Widget,
    pub label: String,
    pub checked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    ButtonPressed(WidgetId),
    CheckBoxChanged(WidgetId, bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiError {
    WidgetIdExhausted,
}

impl std::fmt::Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("the widget ID space is exhausted")
    }
}

impl std::error::Error for UiError {}

#[derive(Debug, Clone, PartialEq)]
enum Control {
    Button(Button),
    CheckBox(CheckBox),
}

impl Control {
    fn widget(&self) -> &Widget {
        match self {
            Self::Button(button) => &button.widget,
            Self::CheckBox(check_box) => &check_box.widget,
        }
    }

    fn set_pressed(&mut self, pressed: bool) {
        if let Self::Button(button) = self {
            button.pressed = pressed;
        }
    }

    fn set_hovered(&mut self, hovered: bool) {
        if let Self::Button(button) = self {
            button.hovered = hovered;
        }
    }
}

/// Stores UI controls and translates pointer snapshots into widget events.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ui {
    controls: Vec<Control>,
    next_id: u64,
    captured: Option<WidgetId>,
}

impl Ui {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_button(
        &mut self,
        bounds: Rect,
        label: impl Into<String>,
    ) -> Result<WidgetId, UiError> {
        let id = self.allocate_id()?;
        self.controls.push(Control::Button(Button {
            widget: Widget::new(id, bounds),
            label: label.into(),
            hovered: false,
            pressed: false,
        }));
        Ok(id)
    }

    pub fn add_checkbox(
        &mut self,
        bounds: Rect,
        label: impl Into<String>,
        checked: bool,
    ) -> Result<WidgetId, UiError> {
        let id = self.allocate_id()?;
        self.controls.push(Control::CheckBox(CheckBox {
            widget: Widget::new(id, bounds),
            label: label.into(),
            checked,
        }));
        Ok(id)
    }

    pub fn button(&self, id: WidgetId) -> Option<&Button> {
        self.controls.iter().find_map(|control| match control {
            Control::Button(button) if button.widget.id == id => Some(button),
            _ => None,
        })
    }

    pub fn button_mut(&mut self, id: WidgetId) -> Option<&mut Button> {
        self.controls.iter_mut().find_map(|control| match control {
            Control::Button(button) if button.widget.id == id => Some(button),
            _ => None,
        })
    }

    pub fn checkbox(&self, id: WidgetId) -> Option<&CheckBox> {
        self.controls.iter().find_map(|control| match control {
            Control::CheckBox(check_box) if check_box.widget.id == id => Some(check_box),
            _ => None,
        })
    }

    pub fn checkbox_mut(&mut self, id: WidgetId) -> Option<&mut CheckBox> {
        self.controls.iter_mut().find_map(|control| match control {
            Control::CheckBox(check_box) if check_box.widget.id == id => Some(check_box),
            _ => None,
        })
    }

    /// Updates hover/pressed state and emits a click only when released inside
    /// the same enabled widget that received the press.
    pub fn update(&mut self, input: &InputSnapshot) -> Vec<UiEvent> {
        let cursor = input.cursor_position();
        for control in &mut self.controls {
            let hovered = cursor.is_some_and(|point| control.widget().contains(point.x, point.y));
            control.set_hovered(hovered);
        }

        if input.mouse_pressed(MouseButton::Left) {
            self.captured = cursor.and_then(|point| {
                self.controls
                    .iter()
                    .rev()
                    .find(|control| control.widget().contains(point.x, point.y))
                    .map(|control| control.widget().id)
            });
            for control in &mut self.controls {
                control.set_pressed(Some(control.widget().id) == self.captured);
            }
        }

        if !input.mouse_released(MouseButton::Left) {
            return Vec::new();
        }
        let Some(captured) = self.captured.take() else {
            return Vec::new();
        };
        let Some(control) = self
            .controls
            .iter_mut()
            .find(|control| control.widget().id == captured)
        else {
            return Vec::new();
        };
        control.set_pressed(false);
        if !cursor.is_some_and(|point| control.widget().contains(point.x, point.y)) {
            return Vec::new();
        }

        match control {
            Control::Button(button) => vec![UiEvent::ButtonPressed(button.widget.id)],
            Control::CheckBox(check_box) => {
                check_box.checked = !check_box.checked;
                vec![UiEvent::CheckBoxChanged(
                    check_box.widget.id,
                    check_box.checked,
                )]
            }
        }
    }

    fn allocate_id(&mut self) -> Result<WidgetId, UiError> {
        let id = self
            .next_id
            .checked_add(1)
            .ok_or(UiError::WidgetIdExhausted)?;
        self.next_id = id;
        Ok(WidgetId(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Vec2;

    fn pointer_input(pressed: bool, released: bool, x: f32) -> InputSnapshot {
        InputSnapshot::new(
            [],
            [],
            [],
            [],
            pressed.then_some(MouseButton::Left),
            released.then_some(MouseButton::Left),
            Some(Vec2::new(x, 12.0)),
        )
    }

    #[test]
    fn clicks_emit_events_only_after_press_and_release_inside() {
        let mut ui = Ui::new();
        let id = ui
            .add_button(Rect::new(0.0, 0.0, 20.0, 20.0), "play")
            .unwrap();
        assert!(ui.update(&pointer_input(true, false, 10.0)).is_empty());
        assert!(ui.button(id).unwrap().pressed);
        assert_eq!(
            ui.update(&pointer_input(false, true, 10.0)),
            vec![UiEvent::ButtonPressed(id)]
        );
        assert!(!ui.button(id).unwrap().pressed);
    }

    #[test]
    fn checkboxes_toggle_on_click_but_not_when_released_outside() {
        let mut ui = Ui::new();
        let id = ui
            .add_checkbox(Rect::new(0.0, 0.0, 20.0, 20.0), "sound", false)
            .unwrap();
        ui.update(&pointer_input(true, false, 10.0));
        assert!(ui.update(&pointer_input(false, true, 30.0)).is_empty());
        assert!(!ui.checkbox(id).unwrap().checked);

        ui.update(&pointer_input(true, false, 10.0));
        assert_eq!(
            ui.update(&pointer_input(false, true, 10.0)),
            vec![UiEvent::CheckBoxChanged(id, true)]
        );
        assert!(ui.checkbox(id).unwrap().checked);
    }
}
