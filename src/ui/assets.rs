//! Texture handles for the fakeflixel UI placeholder graphics.
//!
//! The canonical PNGs are embedded in the crate and decoded at startup.

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use std::collections::HashMap;

const UI_TEXTURES: &[(&str, &[u8])] = &[
    ("box", include_bytes!("../../assets/ui/box.png")),
    ("button", include_bytes!("../../assets/ui/button.png")),
    (
        "button_arrow_down",
        include_bytes!("../../assets/ui/button_arrow_down.png"),
    ),
    (
        "button_arrow_left",
        include_bytes!("../../assets/ui/button_arrow_left.png"),
    ),
    (
        "button_arrow_right",
        include_bytes!("../../assets/ui/button_arrow_right.png"),
    ),
    (
        "button_arrow_up",
        include_bytes!("../../assets/ui/button_arrow_up.png"),
    ),
    (
        "button_thin",
        include_bytes!("../../assets/ui/button_thin.png"),
    ),
    (
        "button_toggle",
        include_bytes!("../../assets/ui/button_toggle.png"),
    ),
    ("check_box", include_bytes!("../../assets/ui/check_box.png")),
    (
        "check_mark",
        include_bytes!("../../assets/ui/check_mark.png"),
    ),
    ("chrome", include_bytes!("../../assets/ui/chrome.png")),
    (
        "chrome_flat",
        include_bytes!("../../assets/ui/chrome_flat.png"),
    ),
    (
        "chrome_inset",
        include_bytes!("../../assets/ui/chrome_inset.png"),
    ),
    (
        "chrome_light",
        include_bytes!("../../assets/ui/chrome_light.png"),
    ),
    (
        "dropdown_mark",
        include_bytes!("../../assets/ui/dropdown_mark.png"),
    ),
    (
        "finger_big",
        include_bytes!("../../assets/ui/finger_big.png"),
    ),
    (
        "finger_small",
        include_bytes!("../../assets/ui/finger_small.png"),
    ),
    ("hilight", include_bytes!("../../assets/ui/hilight.png")),
    ("invis", include_bytes!("../../assets/ui/invis.png")),
    (
        "minus_mark",
        include_bytes!("../../assets/ui/minus_mark.png"),
    ),
    ("plus_mark", include_bytes!("../../assets/ui/plus_mark.png")),
    ("radio", include_bytes!("../../assets/ui/radio.png")),
    ("radio_dot", include_bytes!("../../assets/ui/radio_dot.png")),
    ("swatch", include_bytes!("../../assets/ui/swatch.png")),
    ("tab", include_bytes!("../../assets/ui/tab.png")),
    ("tab_back", include_bytes!("../../assets/ui/tab_back.png")),
    (
        "tooltip_arrow",
        include_bytes!("../../assets/ui/tooltip_arrow.png"),
    ),
];

/// Load a single placeholder PNG as a Bevy image asset.
fn load_texture(name: &str, bytes: &[u8], images: &mut Assets<Image>) -> Handle<Image> {
    let dynamic = image::load_from_memory(bytes)
        .unwrap_or_else(|err| panic!("[fakeflixel::ui] failed to decode {name}: {err}"));
    images.add(Image::from_dynamic(
        dynamic,
        true,
        RenderAssetUsages::default(),
    ))
}

/// All placeholder UI textures, keyed by file name (without extension).
#[derive(Resource, Debug, Clone, Default)]
pub struct FakeFlixelUiAssets {
    pub by_name: HashMap<String, Handle<Image>>,

    /// Button backgrounds.
    pub button: Handle<Image>,
    pub button_thin: Handle<Image>,
    pub button_toggle: Handle<Image>,
    /// Check box / mark.
    pub check_box: Handle<Image>,
    pub check_mark: Handle<Image>,
    /// Radio button / dot.
    pub radio: Handle<Image>,
    pub radio_dot: Handle<Image>,
    /// Window / panel frames.
    pub chrome: Handle<Image>,
    pub chrome_flat: Handle<Image>,
    pub chrome_inset: Handle<Image>,
    pub chrome_light: Handle<Image>,
    /// Tabs.
    pub tab: Handle<Image>,
    pub tab_back: Handle<Image>,
    /// Dropdown indicator.
    pub dropdown_mark: Handle<Image>,
    /// Box / panel filler.
    pub box_: Handle<Image>,
    /// Focus / highlight.
    pub hilight: Handle<Image>,
    /// Transparent hit area.
    pub invis: Handle<Image>,
    /// Stepper marks.
    pub minus_mark: Handle<Image>,
    pub plus_mark: Handle<Image>,
    /// Color swatch.
    pub swatch: Handle<Image>,
    /// Cursors / press visuals.
    pub finger_big: Handle<Image>,
    pub finger_small: Handle<Image>,
    /// Arrow buttons.
    pub button_arrow_down: Handle<Image>,
    pub button_arrow_left: Handle<Image>,
    pub button_arrow_right: Handle<Image>,
    pub button_arrow_up: Handle<Image>,
    /// Tooltip arrow.
    pub tooltip_arrow: Handle<Image>,
}

impl FakeFlixelUiAssets {
    pub fn get(&self, name: &str) -> Option<&Handle<Image>> {
        self.by_name.get(name)
    }
}

/// Startup system: decode every embedded placeholder UI texture.
pub fn load_fakeflixel_ui_assets(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let by_name: HashMap<_, _> = UI_TEXTURES
        .iter()
        .map(|(name, bytes)| ((*name).to_string(), load_texture(name, bytes, &mut images)))
        .collect();

    commands.insert_resource(FakeFlixelUiAssets {
        button: by_name["button"].clone(),
        button_thin: by_name["button_thin"].clone(),
        button_toggle: by_name["button_toggle"].clone(),
        check_box: by_name["check_box"].clone(),
        check_mark: by_name["check_mark"].clone(),
        radio: by_name["radio"].clone(),
        radio_dot: by_name["radio_dot"].clone(),
        chrome: by_name["chrome"].clone(),
        chrome_flat: by_name["chrome_flat"].clone(),
        chrome_inset: by_name["chrome_inset"].clone(),
        chrome_light: by_name["chrome_light"].clone(),
        tab: by_name["tab"].clone(),
        tab_back: by_name["tab_back"].clone(),
        dropdown_mark: by_name["dropdown_mark"].clone(),
        box_: by_name["box"].clone(),
        hilight: by_name["hilight"].clone(),
        invis: by_name["invis"].clone(),
        minus_mark: by_name["minus_mark"].clone(),
        plus_mark: by_name["plus_mark"].clone(),
        swatch: by_name["swatch"].clone(),
        finger_big: by_name["finger_big"].clone(),
        finger_small: by_name["finger_small"].clone(),
        button_arrow_down: by_name["button_arrow_down"].clone(),
        button_arrow_left: by_name["button_arrow_left"].clone(),
        button_arrow_right: by_name["button_arrow_right"].clone(),
        button_arrow_up: by_name["button_arrow_up"].clone(),
        tooltip_arrow: by_name["tooltip_arrow"].clone(),
        by_name,
    });
}
