//! Spawn helpers for fakeflixel UI widgets.
//!
//! All helpers take **screen-space** coordinates: origin at the top-left of the
//! 1280x720 design screen, `y` growing downward (see [`crate::coords`]).
//! Each returns the widget's root [`Entity`]; caption text is spawned as a
//! [`Parent`] child so it follows the widget's transform.

use bevy::hierarchy::BuildChildren;
use bevy::prelude::*;

use crate::coords::{screen_center, screen_rect, screen_to_world};

use super::{
    assets::FakeFlixelUiAssets,
    button::Button,
    checkbox::CheckBox,
    dropdown::{Dropdown, DropdownOption},
    label::Label,
    panel::Panel,
    radio::RadioButton,
    text_input::TextInput,
    widget::{Caption, Widget},
    window::Window,
};

/// Spawn a text caption centered on `parent` (used for widget labels).
fn spawn_caption(
    commands: &mut Commands,
    parent: Entity,
    text: impl Into<String>,
    size: f32,
    color: Color,
) -> Entity {
    let text = text.into();
    let entity = commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    text.clone(),
                    TextStyle {
                        font_size: size,
                        color,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            Caption { owner: parent },
        ))
        .id();
    commands.entity(parent).add_child(entity);
    entity
}

/// Spawn a textured panel (window chrome) at screen-space position.
pub fn spawn_panel(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    z: f32,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.chrome.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, z)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            // Panels are static containers; use `spawn_window` for a
            // draggable, titled window.
            Panel::new("").with_draggable(false),
        ))
        .id()
}

/// Spawn a label (text) at screen-space position.
pub fn spawn_label(
    commands: &mut Commands,
    text: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    size: f32,
    color: Color,
) -> Entity {
    let text = text.into();
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_section(
                    text.clone(),
                    TextStyle {
                        font_size: size,
                        color,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Left),
                transform: Transform::from_xyz(x, -y, 10000.0),
                ..default()
            },
            Widget::new(screen_rect(x, y - size, w, size)),
            Label::new(text).with_size(size).with_color(color),
        ))
        .id()
}

/// Spawn a button with up/over/down textures and a centered caption.
pub fn spawn_button(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    label: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    let label = label.into();
    let entity = commands
        .spawn((
            SpriteBundle {
                texture: assets.button_thin.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, 10000.0)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            Button::new(label.clone()),
        ))
        .id();
    spawn_caption(commands, entity, label, 20.0, Color::WHITE);
    entity
}

/// Spawn a check box with a caption to its right.
/// `size` is the edge length of the box indicator.
pub fn spawn_checkbox(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    label: impl Into<String>,
    x: f32,
    y: f32,
    size: f32,
) -> Entity {
    let label = label.into();
    // Approximate caption width so the widget bounds cover the whole row.
    let caption_w = (label.len() as f32 * size * 0.55).max(size * 4.0);
    let w = size + 6.0 + caption_w;
    let entity = commands
        .spawn((
            SpriteBundle {
                texture: assets.check_box.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_translation(screen_to_world(
                    x + size * 0.5,
                    y + size * 0.5,
                    10000.0,
                )),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, size)),
            CheckBox::new(label.clone()),
        ))
        .id();
    // Caption sits just right of the indicator, centered vertically.
    let caption = spawn_caption(commands, entity, label, size * 0.8, Color::WHITE);
    commands.entity(caption).insert(Transform::from_xyz(
        size * 0.5 + 3.0 + caption_w * 0.5,
        0.0,
        0.0,
    ));
    entity
}

/// Spawn a radio button with a caption to its right.
/// `group` identifies the mutually-exclusive set.
pub fn spawn_radio_button(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    label: impl Into<String>,
    group: impl Into<String>,
    x: f32,
    y: f32,
    size: f32,
) -> Entity {
    let label = label.into();
    let caption_w = (label.len() as f32 * size * 0.55).max(size * 4.0);
    let w = size + 6.0 + caption_w;
    let entity = commands
        .spawn((
            SpriteBundle {
                texture: assets.radio.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_translation(screen_to_world(
                    x + size * 0.5,
                    y + size * 0.5,
                    10000.0,
                )),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, size)),
            RadioButton::new(label.clone(), group),
        ))
        .id();
    let caption = spawn_caption(commands, entity, label, size * 0.8, Color::WHITE);
    commands.entity(caption).insert(Transform::from_xyz(
        size * 0.5 + 3.0 + caption_w * 0.5,
        0.0,
        0.0,
    ));
    entity
}

/// Spawn a single-line text input field.
pub fn spawn_text_input(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.chrome_inset.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, 10000.0)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            TextInput::new(),
        ))
        .id()
}

/// Spawn a dropdown button showing `label`; options are added separately with
/// [`spawn_dropdown_option`].
pub fn spawn_dropdown(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    label: impl Into<String>,
    options: Vec<String>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    let label = label.into();
    let entity = commands
        .spawn((
            SpriteBundle {
                texture: assets.button_thin.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, 10000.0)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            Dropdown::new(label.clone(), options),
        ))
        .id();
    spawn_caption(commands, entity, label, h * 0.6, Color::WHITE);
    entity
}

/// Spawn one option row belonging to `dropdown`.
pub fn spawn_dropdown_option(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    dropdown: Entity,
    index: usize,
    label: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    let label = label.into();
    let entity = commands
        .spawn((
            SpriteBundle {
                texture: assets.box_.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, 10000.0)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            DropdownOption { index, dropdown },
        ))
        .id();
    spawn_caption(commands, entity, label, h * 0.6, Color::WHITE);
    entity
}

/// Spawn a window with title bar (draggable).
pub fn spawn_window(
    commands: &mut Commands,
    assets: &FakeFlixelUiAssets,
    title: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.chrome_light.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_translation(screen_center(x, y, w, h, 10000.0)),
                ..default()
            },
            Widget::new(screen_rect(x, y, w, h)),
            Window::new(title),
        ))
        .id()
}
