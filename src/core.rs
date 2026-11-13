//! Framework-owned game runtime and 2D primitives.

mod assets;
mod input;
mod math;
mod render;
mod runtime;

pub use assets::{AssetError, AssetId, AssetStore, ImageAsset};
pub use input::{InputSnapshot, KeyCode, MouseButton};
pub use math::{Camera, Color, Rect, Vec2, SCREEN_SIZE};
pub use render::{FrameBuffer, FrameBufferError, Renderer, SoftwareRenderer, Sprite};
pub use runtime::{AudioBackend, Backend, Game, GameConfig, GameContext, GameError, Group, State};
