use std::collections::HashMap;

use super::math::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(String);

impl AssetId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AssetId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AssetId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageAsset {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl ImageAsset {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> Result<Self, AssetError> {
        let expected = width
            .checked_mul(height)
            .and_then(|area| usize::try_from(area).ok())
            .ok_or(AssetError::InvalidDimensions)?;
        if width == 0 || height == 0 || pixels.len() != expected {
            return Err(AssetError::InvalidDimensions);
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetError {
    InvalidDimensions,
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => f.write_str("image dimensions do not match its pixel data"),
        }
    }
}

impl std::error::Error for AssetError {}

#[derive(Debug, Default, Clone)]
pub struct AssetStore {
    images: HashMap<AssetId, ImageAsset>,
}

impl AssetStore {
    pub fn insert_image(
        &mut self,
        id: impl Into<AssetId>,
        image: ImageAsset,
    ) -> Option<ImageAsset> {
        self.images.insert(id.into(), image)
    }

    pub fn image(&self, id: &AssetId) -> Option<&ImageAsset> {
        self.images.get(id)
    }

    pub fn remove(&mut self, id: &AssetId) -> Option<ImageAsset> {
        self.images.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_assets_reject_inconsistent_pixel_data() {
        assert_eq!(
            ImageAsset::new(2, 1, vec![Color::WHITE]),
            Err(AssetError::InvalidDimensions)
        );
        assert!(ImageAsset::new(1, 1, vec![Color::WHITE]).is_ok());
    }
}
