use std::{error::Error, time::Duration, time::Instant};

use super::{
    assets::AssetId,
    input::InputSnapshot,
    math::{Color, SCREEN_SIZE},
    render::{FrameBuffer, FrameBufferError, Renderer, SoftwareRenderer},
};

pub trait State: 'static {
    fn update(&mut self, _context: &mut GameContext<'_>) {}

    fn draw(&self, _renderer: &mut dyn Renderer) {}
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub background_color: Color,
    pub update_hz: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            width: SCREEN_SIZE.x as u32,
            height: SCREEN_SIZE.y as u32,
            title: "FakeFlixel".to_owned(),
            background_color: Color::BLACK,
            update_hz: 60,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    InvalidConfiguration,
    FrameBuffer(FrameBufferError),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration => {
                f.write_str("game dimensions and update rate must be non-zero")
            }
            Self::FrameBuffer(error) => error.fmt(f),
        }
    }
}

impl Error for GameError {}

impl From<FrameBufferError> for GameError {
    fn from(value: FrameBufferError) -> Self {
        Self::FrameBuffer(value)
    }
}

pub struct Game {
    config: GameConfig,
    state: Box<dyn State>,
    renderer: SoftwareRenderer,
    accumulator: Duration,
    fixed_step: Duration,
    should_quit: bool,
}

impl Game {
    pub fn new<S: State>(config: GameConfig, initial_state: S) -> Result<Self, GameError> {
        if config.width == 0 || config.height == 0 || config.update_hz == 0 {
            return Err(GameError::InvalidConfiguration);
        }
        Ok(Self {
            renderer: SoftwareRenderer::new(config.width, config.height)?,
            fixed_step: Duration::from_secs_f64(1.0 / f64::from(config.update_hz)),
            config,
            state: Box::new(initial_state),
            accumulator: Duration::ZERO,
            should_quit: false,
        })
    }

    pub fn config(&self) -> &GameConfig {
        &self.config
    }

    pub fn renderer(&self) -> &SoftwareRenderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut SoftwareRenderer {
        &mut self.renderer
    }

    pub fn is_running(&self) -> bool {
        !self.should_quit
    }

    /// Advances the state at a fixed update interval, clamping long pauses.
    /// Returns how many state updates were run.
    pub fn update(&mut self, elapsed: Duration, input: &InputSnapshot) -> usize {
        self.accumulator += elapsed.min(Duration::from_millis(250));
        let mut updates = 0;
        while self.accumulator >= self.fixed_step && !self.should_quit {
            let mut transition = None;
            {
                let mut context = GameContext {
                    delta_seconds: self.fixed_step.as_secs_f32(),
                    input,
                    next_state: &mut transition,
                    should_quit: &mut self.should_quit,
                };
                self.state.update(&mut context);
            }
            if let Some(next_state) = transition {
                self.state = next_state;
            }
            self.accumulator -= self.fixed_step;
            updates += 1;
        }
        updates
    }

    pub fn render(&mut self) {
        self.renderer.clear(self.config.background_color);
        self.state.draw(&mut self.renderer);
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        self.renderer.framebuffer()
    }

    /// Runs the game loop using a platform implementation supplied by the caller.
    pub fn run_with<B: Backend>(&mut self, backend: &mut B) -> Result<(), B::Error> {
        let mut previous_frame = Instant::now();
        while backend.is_open() && self.is_running() {
            let input = backend.poll_input();
            let now = Instant::now();
            self.update(now.duration_since(previous_frame), &input);
            previous_frame = now;
            self.render();
            backend.present(self.framebuffer())?;
        }
        Ok(())
    }
}

pub struct GameContext<'a> {
    delta_seconds: f32,
    input: &'a InputSnapshot,
    next_state: &'a mut Option<Box<dyn State>>,
    should_quit: &'a mut bool,
}

impl GameContext<'_> {
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn input(&self) -> &InputSnapshot {
        self.input
    }

    pub fn transition_to<S: State>(&mut self, state: S) {
        *self.next_state = Some(Box::new(state));
    }

    pub fn quit(&mut self) {
        *self.should_quit = true;
    }
}

/// Interface for a window/input/presentation implementation.
pub trait Backend {
    type Error: Error + 'static;

    fn is_open(&self) -> bool;
    fn poll_input(&mut self) -> InputSnapshot;
    fn present(&mut self, framebuffer: &FrameBuffer) -> Result<(), Self::Error>;
}

/// Audio output is separate from the core runtime and can be implemented by a backend.
pub trait AudioBackend {
    type Error: Error + 'static;

    fn play(&mut self, asset: &AssetId) -> Result<(), Self::Error>;
    fn stop(&mut self, asset: &AssetId) -> Result<(), Self::Error>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Group<T> {
    members: Vec<T>,
}

impl<T> Group<T> {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn add(&mut self, member: T) {
        self.members.push(member);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        (index < self.members.len()).then(|| self.members.remove(index))
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.members.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.members.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{KeyCode, Vec2};

    #[derive(Default)]
    struct Counter;

    impl State for Counter {
        fn update(&mut self, context: &mut GameContext<'_>) {
            assert!((context.delta_seconds() - 0.25).abs() < f32::EPSILON);
            assert!(context.input().key_down(KeyCode::A));
        }

        fn draw(&self, renderer: &mut dyn Renderer) {
            renderer.clear(Color::WHITE);
        }
    }

    struct SwitchAfterOne {
        switched: bool,
    }

    impl State for SwitchAfterOne {
        fn update(&mut self, context: &mut GameContext<'_>) {
            if !self.switched {
                self.switched = true;
                context.transition_to(Counter);
            }
        }
    }

    #[test]
    fn game_uses_fixed_updates_and_clamps_long_pauses() {
        let mut game = Game::new(
            GameConfig {
                update_hz: 4,
                ..GameConfig::default()
            },
            Counter,
        )
        .unwrap();
        let input = InputSnapshot::new([KeyCode::A], [], [], [], [], [], Some(Vec2::ZERO));
        assert_eq!(game.update(Duration::from_millis(100), &input), 0);
        assert_eq!(game.update(Duration::from_millis(250), &input), 1);
    }

    #[test]
    fn state_transitions_apply_after_the_current_update() {
        let mut game = Game::new(
            GameConfig {
                update_hz: 4,
                ..GameConfig::default()
            },
            SwitchAfterOne { switched: false },
        )
        .unwrap();
        game.update(Duration::from_millis(250), &InputSnapshot::default());
        game.update(
            Duration::from_millis(250),
            &InputSnapshot::new([KeyCode::A], [], [], [], [], [], None),
        );
        game.render();
        assert_eq!(game.framebuffer().pixel(0, 0), Some(Color::WHITE));
    }

    #[test]
    fn groups_remove_by_index_without_reordering_the_rest() {
        let mut group = Group::new();
        group.add(1);
        group.add(2);
        group.add(3);
        assert_eq!(group.remove(1), Some(2));
        assert_eq!(group.iter().copied().collect::<Vec<_>>(), vec![1, 3]);
        assert_eq!(group.remove(5), None);
    }
}
