use bevy::{
    prelude::*, render::texture::ImagePlugin, window::WindowResolution, winit::WinitSettings,
};

// dark purple #25131a = 39/255, 19/255, 26/255
const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

const HEIGHT: f32 = 720.0;
const RESOLUTION: f32 = 16.0 / 9.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
        )
        .add_startup_systems((setup, spawn_camera))
        .add_system(button_system);

    app.run();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Scene"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::height(Val::Percent(50.)),
                            flex_direction: FlexDirection::Row,
                            // horizontally center child portrait
                            justify_content: JustifyContent::Center,
                            // vertically center child portrait
                            align_items: AlignItems::Center,
                            gap: Size::width(Val::Percent(5.)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Interlocutor Choices"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("textures/character/Icons_12.png"),
                                flip_x: true,
                                ..default()
                            },
                            style: Style {
                                size: Size::width(Val::Percent(10.)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Old Frog Portrait"),
                    ));

                    parent.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("textures/character/Icons_23.png"),
                                flip_x: true,
                                ..default()
                            },
                            style: Style {
                                size: Size::width(Val::Percent(10.)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Frog Portrait"),
                    ));

                    parent.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("textures/character/Icons_27.png"),
                                flip_x: true,
                                ..default()
                            },
                            style: Style {
                                size: Size::width(Val::Percent(10.)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Warrior Frog Portrait"),
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::height(Val::Percent(50.)),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Dialog Section"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(15.)),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Interlocutor Portrait NPC"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server
                                            .load("textures/character/background.png"),
                                        flip_x: true,
                                        ..default()
                                    },
                                    style: Style {
                                        // size: Size::all(Val::Px(50.)),
                                        size: Size::width(Val::Percent(100.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Portrait"),
                            ));
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    size: Size::width(Val::Percent(70.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Dialog Panel"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            // TODO: Bevy 0.11 default font
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_alignment(TextAlignment::Left),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        // TODO: Text Style
                                        margin: UiRect {
                                            left: Val::Percent(24.),
                                            ..default()
                                        },
                                        size: Size::width(Val::Percent(50.)),
                                        align_content: AlignContent::SpaceAround,
                                        align_self: AlignSelf::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Dialog NPC"),
                            ));

                            parent
                                .spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                // TODO: Bevy 0.11 default font
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 30.,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Left),
                                        style: Style {
                                            size: Size::width(Val::Percent(50.)),
                                            flex_direction: FlexDirection::Column,
                                            align_content: AlignContent::SpaceAround,
                                            // horizontally center child choices
                                            justify_content: JustifyContent::Center,
                                            // vertically center child choices
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            // TODO: Text Style
                                            margin: UiRect {
                                                left: Val::Percent(24.),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Dialog Player"),
                                ))
                                .with_children(|parent| {
                                    for i in 0..3 {
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Px(150.),
                                                            Val::Px(65.),
                                                        ),
                                                        // horizontally center child text
                                                        justify_content: JustifyContent::Center,
                                                        // vertically center child text
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    background_color: NORMAL_BUTTON.into(),
                                                    visibility: Visibility::Hidden,
                                                    ..default()
                                                },
                                                Name::new(format!("Button {i}")),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "Button",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 30.,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ));
                                            });
                                    }
                                });
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(15.)),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Interlocutor Portrait Player"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: asset_server
                                        .load("textures/character/Icons_05.png")
                                        .into(),
                                    style: Style {
                                        // size: Size::all(Val::Px(50.)),
                                        size: Size::width(Val::Percent(100.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Portrait"),
                            ));
                        });
                });
        });
}
