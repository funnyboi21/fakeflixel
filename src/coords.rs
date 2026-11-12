//! Screen-space coordinate helpers for fakeflixel widgets.
//!
//! fakeflixel uses Flixel-style coordinates: origin at the **top-left** of the
//! design screen (1280x720) with `y` growing
//! downward. Bevy world space is centered with `y` growing upward, so every
//! spawn helper must convert through [`screen_to_world`].

use bevy::prelude::*;

/// FakeFlixel's default design resolution.
pub const SCREEN_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

/// Convert a top-left-origin screen point to Bevy world space.
#[inline]
pub fn screen_to_world(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, -y, z)
}

/// Convert a top-left-origin screen point to a Bevy world-space `Vec2`.
#[inline]
pub fn screen_to_world2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, -y)
}

/// Logical screen-space bounds for a widget at `(x, y)` with size `(w, h)`.
/// `y` is the distance from the top of the screen.
#[inline]
pub fn screen_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect {
        min: Vec2::new(x, y),
        max: Vec2::new(x + w, y + h),
    }
}

/// World-space position of the **center** of a screen-space rect.
#[inline]
pub fn screen_center(x: f32, y: f32, w: f32, h: f32, z: f32) -> Vec3 {
    screen_to_world(x + w * 0.5, y + h * 0.5, z)
}

/// Horizontal screen position that centers `width` pixels on the screen.
#[inline]
pub fn centered_x(width: f32) -> f32 {
    (SCREEN_SIZE.x - width) * 0.5
}

/// Vertical screen position that centers `height` pixels on the screen.
#[inline]
pub fn centered_y(height: f32) -> f32 {
    (SCREEN_SIZE.y - height) * 0.5
}
