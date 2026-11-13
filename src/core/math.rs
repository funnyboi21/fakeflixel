use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// Default logical design resolution, with a top-left origin and downward y.
pub const SCREEN_SIZE: Vec2 = Vec2::new(1280.0, 720.0);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgba(0, 0, 0, 255);
    pub const WHITE: Self = Self::rgba(255, 255, 255, 255);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub(crate) fn blend_over(self, background: Self) -> Self {
        let alpha = u16::from(self.a);
        let inverse_alpha = 255 - alpha;
        let blend = |foreground: u8, background: u8| {
            ((u16::from(foreground) * alpha + u16::from(background) * inverse_alpha) / 255) as u8
        };
        Self::rgb(
            blend(self.r, background.r),
            blend(self.g, background.g),
            blend(self.b, background.b),
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

/// A 2D camera whose viewport is expressed in screen coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub scroll: Vec2,
    pub zoom: f32,
    pub viewport: Rect,
}

impl Camera {
    pub const fn new(viewport: Rect) -> Self {
        Self {
            scroll: Vec2::ZERO,
            zoom: 1.0,
            viewport,
        }
    }

    pub fn world_to_screen(self, point: Vec2) -> Vec2 {
        self.viewport.min + (point - self.scroll) * self.zoom
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Rect::new(0.0, 0.0, SCREEN_SIZE.x, SCREEN_SIZE.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_keep_the_top_left_origin() {
        let camera = Camera {
            scroll: Vec2::new(10.0, 20.0),
            zoom: 2.0,
            viewport: Rect::new(100.0, 50.0, 640.0, 360.0),
        };
        assert_eq!(
            camera.world_to_screen(Vec2::new(20.0, 30.0)),
            Vec2::new(120.0, 70.0)
        );
        assert_eq!(SCREEN_SIZE, Vec2::new(1280.0, 720.0));
    }
}
