use std::collections::HashSet;

use minifb::{Key, MouseMode, Window, WindowOptions};

use crate::core::{Backend, Game, GameConfig, InputSnapshot, KeyCode, MouseButton, Vec2};

pub struct MinifbBackend {
    window: Window,
    previous_keys: HashSet<KeyCode>,
    previous_mouse: HashSet<MouseButton>,
}

impl MinifbBackend {
    pub fn new(config: &GameConfig) -> Result<Self, minifb::Error> {
        let window = Window::new(
            &config.title,
            config.width as usize,
            config.height as usize,
            WindowOptions::default(),
        )?;
        Ok(Self {
            window,
            previous_keys: HashSet::new(),
            previous_mouse: HashSet::new(),
        })
    }
}

impl Backend for MinifbBackend {
    type Error = minifb::Error;

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn poll_input(&mut self) -> InputSnapshot {
        let keys_down: HashSet<_> = self
            .window
            .get_keys()
            .into_iter()
            .filter_map(map_key)
            .collect();
        let keys_pressed = keys_down
            .difference(&self.previous_keys)
            .copied()
            .collect::<Vec<_>>();
        let keys_released = self
            .previous_keys
            .difference(&keys_down)
            .copied()
            .collect::<Vec<_>>();

        let mut mouse_down = HashSet::new();
        for (minifb_button, button) in [
            (minifb::MouseButton::Left, MouseButton::Left),
            (minifb::MouseButton::Right, MouseButton::Right),
            (minifb::MouseButton::Middle, MouseButton::Middle),
        ] {
            if self.window.get_mouse_down(minifb_button) {
                mouse_down.insert(button);
            }
        }
        let mouse_pressed = mouse_down
            .difference(&self.previous_mouse)
            .copied()
            .collect::<Vec<_>>();
        let mouse_released = self
            .previous_mouse
            .difference(&mouse_down)
            .copied()
            .collect::<Vec<_>>();
        let cursor = self
            .window
            .get_mouse_pos(MouseMode::Pass)
            .map(|(x, y)| Vec2::new(x, y));

        self.previous_keys = keys_down.clone();
        self.previous_mouse = mouse_down.clone();
        InputSnapshot::new(
            keys_down,
            keys_pressed,
            keys_released,
            mouse_down,
            mouse_pressed,
            mouse_released,
            cursor,
        )
    }

    fn present(&mut self, framebuffer: &crate::core::FrameBuffer) -> Result<(), Self::Error> {
        self.window.update_with_buffer(
            framebuffer.pixels(),
            framebuffer.width() as usize,
            framebuffer.height() as usize,
        )
    }
}

impl Game {
    /// Creates a native window and runs this game using the minifb backend.
    pub fn run(mut self) -> Result<(), minifb::Error> {
        let mut backend = MinifbBackend::new(self.config())?;
        self.run_with(&mut backend)
    }
}

fn map_key(key: Key) -> Option<KeyCode> {
    Some(match key {
        Key::A => KeyCode::A,
        Key::B => KeyCode::B,
        Key::C => KeyCode::C,
        Key::D => KeyCode::D,
        Key::E => KeyCode::E,
        Key::F => KeyCode::F,
        Key::G => KeyCode::G,
        Key::H => KeyCode::H,
        Key::I => KeyCode::I,
        Key::J => KeyCode::J,
        Key::K => KeyCode::K,
        Key::L => KeyCode::L,
        Key::M => KeyCode::M,
        Key::N => KeyCode::N,
        Key::O => KeyCode::O,
        Key::P => KeyCode::P,
        Key::Q => KeyCode::Q,
        Key::R => KeyCode::R,
        Key::S => KeyCode::S,
        Key::T => KeyCode::T,
        Key::U => KeyCode::U,
        Key::V => KeyCode::V,
        Key::W => KeyCode::W,
        Key::X => KeyCode::X,
        Key::Y => KeyCode::Y,
        Key::Z => KeyCode::Z,
        Key::Key0 => KeyCode::Digit0,
        Key::Key1 => KeyCode::Digit1,
        Key::Key2 => KeyCode::Digit2,
        Key::Key3 => KeyCode::Digit3,
        Key::Key4 => KeyCode::Digit4,
        Key::Key5 => KeyCode::Digit5,
        Key::Key6 => KeyCode::Digit6,
        Key::Key7 => KeyCode::Digit7,
        Key::Key8 => KeyCode::Digit8,
        Key::Key9 => KeyCode::Digit9,
        Key::Enter => KeyCode::Enter,
        Key::Escape => KeyCode::Escape,
        Key::Space => KeyCode::Space,
        Key::Backspace => KeyCode::Backspace,
        Key::Tab => KeyCode::Tab,
        Key::Left => KeyCode::Left,
        Key::Right => KeyCode::Right,
        Key::Up => KeyCode::Up,
        Key::Down => KeyCode::Down,
        Key::LeftShift => KeyCode::LeftShift,
        Key::RightShift => KeyCode::RightShift,
        Key::LeftCtrl => KeyCode::LeftControl,
        Key::RightCtrl => KeyCode::RightControl,
        _ => return None,
    })
}
