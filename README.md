# FakeFlixel

FakeFlixel is a HaxeFlixel-inspired UI framework for Bevy 0.14. It provides
buttons, check boxes, radio buttons, text inputs, dropdowns, panels, windows,
shared focus and theme resources, and Flixel-style screen-coordinate helpers.

Add it to a Bevy project with:

```toml
[dependencies]
fakeflixel = { git = "https://github.com/funnyboi21/fakeflixel" }
```

Register its plugin once:

```rust
use bevy::prelude::*;
use fakeflixel::FakeFlixelPlugin;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FakeFlixelPlugin);
```

The UI texture assets are embedded in the crate, so applications do not need
to copy a separate asset directory.
