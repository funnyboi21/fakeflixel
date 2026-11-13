use super::{
    assets::{AssetId, AssetStore},
    math::{Camera, Color, Rect, Vec2},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub asset: Option<AssetId>,
    pub color: Color,
    pub visible: bool,
}

impl Sprite {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(width, height),
            velocity: Vec2::ZERO,
            asset: None,
            color: Color::WHITE,
            visible: true,
        }
    }

    pub fn from_asset(asset: impl Into<AssetId>, width: f32, height: f32) -> Self {
        Self {
            asset: Some(asset.into()),
            ..Self::new(width, height)
        }
    }

    pub fn update(&mut self, delta_seconds: f32) {
        self.position += self.velocity * delta_seconds;
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

/// Output surface for drawing game objects in screen-space coordinates.
pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn draw_sprite(&mut self, sprite: &Sprite);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBuffer {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Result<Self, FrameBufferError> {
        let length = width
            .checked_mul(height)
            .and_then(|area| usize::try_from(area).ok())
            .ok_or(FrameBufferError::InvalidDimensions)?;
        if width == 0 || height == 0 {
            return Err(FrameBufferError::InvalidDimensions);
        }
        Ok(Self {
            width,
            height,
            pixels: vec![0; length],
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Pixels are 0x00RRGGBB, the format expected by minifb.
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let pixel = self.pixels[(y * self.width + x) as usize];
        Some(Color::rgb(
            ((pixel >> 16) & 0xff) as u8,
            ((pixel >> 8) & 0xff) as u8,
            (pixel & 0xff) as u8,
        ))
    }

    fn blend_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = (y * self.width + x) as usize;
        let background = self.pixels[index];
        let blended = color.blend_over(Color::rgb(
            ((background >> 16) & 0xff) as u8,
            ((background >> 8) & 0xff) as u8,
            (background & 0xff) as u8,
        ));
        self.pixels[index] =
            (u32::from(blended.r) << 16) | (u32::from(blended.g) << 8) | u32::from(blended.b);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameBufferError {
    InvalidDimensions,
}

impl std::fmt::Display for FrameBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("framebuffer dimensions must be non-zero and fit in memory")
    }
}

impl std::error::Error for FrameBufferError {}

/// A dependency-free CPU renderer used by the headless core and native backends.
#[derive(Debug, Clone)]
pub struct SoftwareRenderer {
    framebuffer: FrameBuffer,
    assets: AssetStore,
    camera: Camera,
}

impl SoftwareRenderer {
    pub fn new(width: u32, height: u32) -> Result<Self, FrameBufferError> {
        Ok(Self {
            framebuffer: FrameBuffer::new(width, height)?,
            assets: AssetStore::default(),
            camera: Camera::default(),
        })
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        &self.framebuffer
    }

    pub fn assets(&self) -> &AssetStore {
        &self.assets
    }

    pub fn assets_mut(&mut self) -> &mut AssetStore {
        &mut self.assets
    }

    pub fn camera(&self) -> Camera {
        self.camera
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    fn draw_image(&mut self, sprite: &Sprite) {
        let Some(asset_id) = sprite.asset.as_ref() else {
            self.draw_rect(
                Rect::new(
                    sprite.position.x,
                    sprite.position.y,
                    sprite.size.x,
                    sprite.size.y,
                ),
                sprite.color,
            );
            return;
        };
        let Some(image) = self.assets.image(asset_id) else {
            return;
        };
        if sprite.size.x <= 0.0 || sprite.size.y <= 0.0 || self.camera.zoom <= 0.0 {
            return;
        }

        let screen_position = self.camera.world_to_screen(sprite.position);
        let width = sprite.size.x * self.camera.zoom;
        let height = sprite.size.y * self.camera.zoom;
        let start_x = screen_position
            .x
            .floor()
            .max(0.0)
            .max(self.camera.viewport.min.x.floor()) as u32;
        let start_y = screen_position
            .y
            .floor()
            .max(0.0)
            .max(self.camera.viewport.min.y.floor()) as u32;
        let end_x = (screen_position.x + width)
            .ceil()
            .min(self.framebuffer.width as f32)
            .min(self.camera.viewport.max.x.ceil())
            .max(0.0) as u32;
        let end_y = (screen_position.y + height)
            .ceil()
            .min(self.framebuffer.height as f32)
            .min(self.camera.viewport.max.y.ceil())
            .max(0.0) as u32;

        let image_width = image.width();
        let image_height = image.height();
        for y in start_y..end_y {
            let source_y = (((y as f32 - screen_position.y) / height) * image_height as f32) as u32;
            for x in start_x..end_x {
                let source_x =
                    (((x as f32 - screen_position.x) / width) * image_width as f32) as u32;
                let source = image.pixels()[(source_y.min(image_height - 1) * image_width
                    + source_x.min(image_width - 1))
                    as usize];
                let tinted = Color::rgba(
                    ((u16::from(source.r) * u16::from(sprite.color.r)) / 255) as u8,
                    ((u16::from(source.g) * u16::from(sprite.color.g)) / 255) as u8,
                    ((u16::from(source.b) * u16::from(sprite.color.b)) / 255) as u8,
                    ((u16::from(source.a) * u16::from(sprite.color.a)) / 255) as u8,
                );
                self.framebuffer.blend_pixel(x, y, tinted);
            }
        }
    }
}

impl Renderer for SoftwareRenderer {
    fn clear(&mut self, color: Color) {
        let rgb = (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b);
        self.framebuffer.pixels.fill(rgb);
    }

    fn draw_rect(&mut self, rect: Rect, color: Color) {
        if rect.width() <= 0.0 || rect.height() <= 0.0 || self.camera.zoom <= 0.0 {
            return;
        }
        let min = self.camera.world_to_screen(rect.min);
        let max = self.camera.world_to_screen(rect.max);
        let start_x = min
            .x
            .floor()
            .max(0.0)
            .max(self.camera.viewport.min.x.floor()) as u32;
        let start_y = min
            .y
            .floor()
            .max(0.0)
            .max(self.camera.viewport.min.y.floor()) as u32;
        let end_x = max
            .x
            .ceil()
            .min(self.framebuffer.width as f32)
            .min(self.camera.viewport.max.x.ceil())
            .max(0.0) as u32;
        let end_y = max
            .y
            .ceil()
            .min(self.framebuffer.height as f32)
            .min(self.camera.viewport.max.y.ceil())
            .max(0.0) as u32;
        for y in start_y..end_y {
            for x in start_x..end_x {
                self.framebuffer.blend_pixel(x, y, color);
            }
        }
    }

    fn draw_sprite(&mut self, sprite: &Sprite) {
        if sprite.visible {
            self.draw_image(sprite);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::assets::ImageAsset;

    #[test]
    fn software_renderer_clips_and_blends_rectangles() {
        let mut renderer = SoftwareRenderer::new(4, 3).unwrap();
        renderer.clear(Color::BLACK);
        renderer.draw_rect(Rect::new(-1.0, 1.0, 3.0, 2.0), Color::rgba(255, 0, 0, 128));
        assert_eq!(
            renderer.framebuffer().pixel(0, 1),
            Some(Color::rgb(128, 0, 0))
        );
        assert_eq!(
            renderer.framebuffer().pixel(1, 2),
            Some(Color::rgb(128, 0, 0))
        );
        assert_eq!(renderer.framebuffer().pixel(2, 1), Some(Color::BLACK));
    }

    #[test]
    fn sprites_draw_their_registered_image_with_camera_transform() {
        let mut renderer = SoftwareRenderer::new(4, 4).unwrap();
        renderer.clear(Color::BLACK);
        renderer
            .assets_mut()
            .insert_image("pixel", ImageAsset::new(1, 1, vec![Color::WHITE]).unwrap());
        renderer.set_camera(Camera {
            scroll: Vec2::new(1.0, 1.0),
            zoom: 2.0,
            viewport: Rect::new(0.0, 0.0, 4.0, 4.0),
        });
        let mut sprite = Sprite::from_asset("pixel", 1.0, 1.0);
        sprite.position = Vec2::new(1.0, 1.0);
        renderer.draw_sprite(&sprite);
        assert_eq!(renderer.framebuffer().pixel(0, 0), Some(Color::WHITE));
        assert_eq!(renderer.framebuffer().pixel(1, 1), Some(Color::WHITE));
        assert_eq!(renderer.framebuffer().pixel(2, 2), Some(Color::BLACK));
    }
}
