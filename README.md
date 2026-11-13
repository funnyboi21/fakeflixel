# FakeFlixel

FakeFlixel is a Flixel-inspired 2D game framework. Its default core owns the
game/state API, screen-space math, input snapshots, assets, a software renderer,
and a fixed-step update loop; it has no third-party dependencies and can be
tested headlessly. Coordinates use a top-left origin with a 1280×720 default
design resolution.

```toml
[dependencies]
fakeflixel = { git = "https://github.com/funnyboi21/fakeflixel", features = ["minifb"] }
```

```rust
use fakeflixel::{Color, Game, GameConfig, GameContext, Renderer, Sprite, State};

struct PlayState {
    player: Sprite,
}

impl State for PlayState {
    fn update(&mut self, context: &mut GameContext<'_>) {
        if context.input().key_down(fakeflixel::KeyCode::Right) {
            self.player.position.x += 120.0 * context.delta_seconds();
        }
    }

    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.clear(Color::BLACK);
        renderer.draw_sprite(&self.player);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Game::new(
        GameConfig::default(),
        PlayState {
            player: Sprite::new(32.0, 32.0),
        },
    )?
    .run()?;
    Ok(())
}
```

The optional `minifb` feature provides a small native window/input backend.
The backend and the core are compiled into the game; players do not need a
separate Bevy runtime. To keep using the existing Bevy 0.14 widgets during
migration, enable `bevy-ui` and register `FakeFlixelPlugin` in a Bevy app.

The current Bevy widgets remain a compatibility layer; their behavior and
embedded textures are being replaced incrementally with framework-owned widget
IDs and state.
