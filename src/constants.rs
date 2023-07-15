//! Constants
//!
//! 1 == one pixel
//! 0.6 == characters' pixel
//! magical number = ratio

pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

pub const HEIGHT: f32 = 720.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

pub mod character {
    pub const KARMA_MIN: i32 = -100;
    pub const KARMA_MAX: i32 = 100;

    pub mod dialog {
        // Flibittygibbit

        // TODO: feature - Read at dialog_file instead of CST
        // CST = path to the file

        pub const RANDOM_DIALOG: &str = "# Fabien\n
- Enfant, j'ai eu un poney
- Mais j'ai toujours voulu un agneau\n";
        pub const OLF_DIALOG: &str = "# Olf

- Il faut absolument sauver les Fabien du Chien Geant

## Morgan

- ... | None

### Olf

- Il me faut donc obtenir le trone

#### Morgan

- ... | None
- et de l'$ | None

##### Olf

- Et de l'$
- C'est essentiel

##### Olf

- C'est essentiel\n";
        pub const FABIEN_DIALOG: &str = "# Fabien

- Hello

## Fabien

- /<3

### Morgan

- Hey | None
- No Hello | None
- Want to share a flat ? | None

#### Fabien

- :)

#### Fabien

- :O

#### Fabien

- Sure\n";

        pub const MORGAN_DIALOG: &str =
            "# Morgan\n\n- Bonjour Florian. /\nComment vas/-tu ? /\nJ'ai faim.\n";
    }
}

pub mod ui {
    pub mod dialogs {
        use bevy::prelude::Color;

        pub const DIALOG_PANEL_ANIMATION_OFFSET: f32 = -1000.0;
        pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
        pub const DIALOG_PANEL_ANIMATION_TIME_MS: u64 = 500;
        pub const SCROLL_SIZE: (f32, f32) = (490.0, 11700.0 / 45.0);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

        pub const TRANSPARENT_BUTTON: Color = Color::rgba(0., 0., 0., 0.);
        // pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
        pub const NORMAL_BUTTON: Color = Color::rgba(0.01, 0.01, 0.01, 0.01);
        pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    }
}
