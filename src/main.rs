use bevy::{prelude::*, render::texture::ImagePlugin, window::WindowResolution};
use bevy_tweening::TweeningPlugin;
use ui::dialog_system::Dialog;

use crate::ui::UiPlugin;
use constants::{character::dialog::OLF_DIALOG, CLEAR, FIXED_TIME_STEP, HEIGHT, RESOLUTION};

pub mod constants;
pub mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    Discussion,
}

fn main() {
    // // When building for WASM, print panics to the browser console
    // #[cfg(target_arch = "wasm32")]
    // console_error_panic_hook::set_once();

    let mut app = App::new();
    app.add_state::<GameState>()
        .insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                        title: "Bevy Dialog".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TweeningPlugin)
        // .add_plugin(DebugPlugin)
        .add_plugin(UiPlugin)
        .add_startup_systems((spawn_camera, spawn_player));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

/// TEMP: CUSTOM
fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Dialog::new(OLF_DIALOG), Name::new("Temp player")));
}

#[derive(Component)]
pub struct Karma(pub i32);

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct Player;
