use std::collections::HashSet;

use super::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Enter,
    Escape,
    Space,
    Backspace,
    Tab,
    Left,
    Right,
    Up,
    Down,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Immutable per-frame input supplied by a platform backend.
#[derive(Debug, Clone, Default)]
pub struct InputSnapshot {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    cursor: Option<Vec2>,
}

impl InputSnapshot {
    pub fn new(
        keys_down: impl IntoIterator<Item = KeyCode>,
        keys_pressed: impl IntoIterator<Item = KeyCode>,
        keys_released: impl IntoIterator<Item = KeyCode>,
        mouse_down: impl IntoIterator<Item = MouseButton>,
        mouse_pressed: impl IntoIterator<Item = MouseButton>,
        mouse_released: impl IntoIterator<Item = MouseButton>,
        cursor: Option<Vec2>,
    ) -> Self {
        Self {
            keys_down: keys_down.into_iter().collect(),
            keys_pressed: keys_pressed.into_iter().collect(),
            keys_released: keys_released.into_iter().collect(),
            mouse_down: mouse_down.into_iter().collect(),
            mouse_pressed: mouse_pressed.into_iter().collect(),
            mouse_released: mouse_released.into_iter().collect(),
            cursor,
        }
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    pub fn cursor_position(&self) -> Option<Vec2> {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshots_distinguish_held_and_edge_inputs() {
        let input = InputSnapshot::new(
            [KeyCode::A],
            [KeyCode::A],
            [],
            [MouseButton::Left],
            [],
            [],
            Some(Vec2::new(12.0, 34.0)),
        );
        assert!(input.key_down(KeyCode::A));
        assert!(input.key_pressed(KeyCode::A));
        assert!(!input.key_released(KeyCode::A));
        assert!(input.mouse_down(MouseButton::Left));
        assert_eq!(input.cursor_position(), Some(Vec2::new(12.0, 34.0)));
    }
}
