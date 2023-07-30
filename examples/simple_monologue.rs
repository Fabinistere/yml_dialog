//! Simple Example of a monologue.

use bevy::{
    prelude::*, render::texture::ImagePlugin, window::WindowResolution, winit::WinitSettings,
};

// dark purple #25131a = 39/255, 19/255, 26/255
const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

const HEIGHT: f32 = 720.0;
const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa::Off)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::game())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                        title: "Complete Dialog".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

    app.run();
}
