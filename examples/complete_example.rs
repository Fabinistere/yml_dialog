//! Complete Example of a three Dialog.
//!
//! - Press any key to continue the dialog.
//! - Choose your answer with the down right buttons.
//! - You can press the reset button to ... to reset.
//! - Click on one of the three frog portrait above.

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::texture::ImagePlugin,
    window::WindowResolution,
    winit::WinitSettings,
};
use rand::seq::SliceRandom;
use std::{fmt, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use fto_dialog::{init_tree_file, DialogContent, DialogCustomInfos};

// dark purple #25131a = 39/255, 19/255, 26/255
const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

const HEIGHT: f32 = 720.0;
const RESOLUTION: f32 = 16.0 / 9.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Reflect, Deref, DerefMut, Clone, Default, Resource)]
struct CurrentInterlocutor {
    interlocutor: Option<Entity>,
}

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Deref, DerefMut, Clone, Default, Resource)]
struct ActiveWorldEvents {
    active_world_events: Vec<WorldEvent>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter)]
enum WorldEvent {
    FrogLove,
    FrogHate,
    FrogTalk,
}

impl fmt::Display for WorldEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorldEvent::FrogLove => write!(f, "FrogLove"),
            WorldEvent::FrogHate => write!(f, "FrogHate"),
            WorldEvent::FrogTalk => write!(f, "FrogTalk"),
        }
    }
}

impl FromStr for WorldEvent {
    type Err = ();

    fn from_str(input: &str) -> Result<WorldEvent, Self::Err> {
        match input {
            "FrogTalk" => Ok(WorldEvent::FrogTalk),
            "FrogLove" => Ok(WorldEvent::FrogLove),
            "FrogHate" => Ok(WorldEvent::FrogHate),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default, Component)]
struct Dialog {
    root: Option<String>,
    current_node: Option<String>,
}

impl Dialog {
    fn all(root: Option<String>) -> Self {
        Dialog {
            root: root.clone(),
            current_node: root,
        }
    }

    // fn root(&self) -> Option<String> {
    //     self.root.clone()
    // }

    // fn set_current(&mut self, current_node: Option<String>) {
    //     self.current_node = current_node
    // }
}

/// Points to a interactable portrait.
#[derive(Component)]
struct Portrait;

/// Points to the NPC portrait on the dialog Panel.
#[derive(Component)]
struct InterlocutorPortait;

/// Contains the index of the choice.
#[derive(
    Debug, Reflect, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Component,
)]
struct Choice(usize);

#[derive(Component)]
struct Reset;

#[derive(Component)]
struct PlayerPanel;

#[derive(Component)]
struct NPCPanel;

// TODO: Visual - DialogPanel Seperator + background

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
        .insert_resource(CurrentInterlocutor::default())
        .insert_resource(ActiveWorldEvents::default())
        .add_event::<DialogDiveEvent>()
        .add_event::<TriggerEvents>()
        .add_startup_systems((setup, spawn_camera))
        .add_systems((
            continue_dialog,
            choose_answer,
            reset_system,
            switch_dialog,
            dialog_dive,
            update_dialog_panel,
            trigger_event_handler.after(dialog_dive),
            change_interlocutor_portrait,
            button_system,
            // button_visibility,
        ));

    app.run();
}

fn reset_system(
    mut active_world_events: ResMut<ActiveWorldEvents>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Reset>, With<Button>)>,
    mut dialog_query: Query<&mut Dialog, With<Portrait>>,
) {
    if let Ok(interaction) = interaction_query.get_single() {
        if *interaction == Interaction::Clicked {
            for mut dialog in &mut dialog_query {
                dialog.current_node = dialog.root.clone()
            }
            active_world_events.clear();
        }
    }
}

fn switch_dialog(
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Portrait>)>,
    mut current_interlocutor: ResMut<CurrentInterlocutor>,
) {
    for (portrait, interaction) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            // info!("Switch Interlocutor");
            current_interlocutor.interlocutor = Some(portrait);
        }
    }
}

fn choose_answer(
    choice_query: Query<(&Choice, &Interaction), Changed<Interaction>>,
    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
) {
    for (choice_index, interaction) in &choice_query {
        if *interaction == Interaction::Clicked {
            dialog_dive_event.send(DialogDiveEvent {
                child_index: **choice_index,
                skip: false,
            });
        }
    }
}

/// Happens when
///   - `continue_dialog()`
///     - any key pressed
///
/// Read in
///   - `dialog_dive()`
///     - analyze the current node;
///     If not empty,
///       - drop until there is 1 or less text in the UpeerScroll
///       OR
///       - go down to the correct child index
pub struct DialogDiveEvent {
    pub child_index: usize,
    pub skip: bool,
}

fn continue_dialog(
    mut key_evr: EventReader<KeyboardInput>,
    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
) {
    for ev in key_evr.iter() {
        if ev.state == ButtonState::Pressed {
            dialog_dive_event.send(DialogDiveEvent {
                child_index: 0,
                skip: true,
            });
        }
        // break;
    }
}

/// Happens when
///   - `dialog_dive()`
///     - when leaving a node
///
/// Read in
///   - `trigger_event_handler()`
///     - If the event is not already active
///     add it to the WorldEvent list.
struct TriggerEvents(Vec<String>);

fn trigger_event_handler(
    mut trigger_event: EventReader<TriggerEvents>,
    mut active_world_events: ResMut<ActiveWorldEvents>,
) {
    for TriggerEvents(incomming_events) in trigger_event.iter() {
        for event_to_trigger in incomming_events {
            match WorldEvent::from_str(&event_to_trigger) {
                Err(_) => {}
                Ok(event) => {
                    if !active_world_events.contains(&event) {
                        active_world_events.push(event)
                    }
                }
            }
        }
    }
}

/// Analyze the current node;
///
/// If not empty,
/// - drop until there is 1 or less text
/// - go down to the correct child index
///
/// # Note
///
/// Every modification of the DialogPanel's content
/// will modify the dialog contained the concerned interlocutor
fn dialog_dive(
    mut dialog_dive_event: EventReader<DialogDiveEvent>,
    current_interlocutor: Res<CurrentInterlocutor>,
    mut dialog_query: Query<&mut Dialog, With<Portrait>>,

    mut trigger_event: EventWriter<TriggerEvents>,
) {
    for DialogDiveEvent { child_index, skip } in dialog_dive_event.iter() {
        // info!("DEBUG: DialogDive Event");
        match current_interlocutor.interlocutor {
            None => {}
            Some(_interlocutor) => {
                let mut dialog = dialog_query.get_mut(current_interlocutor.unwrap()).unwrap();

                match &dialog.current_node {
                    None => {}
                    Some(current_node_string) => {
                        let dialog_tree = init_tree_file(
                            current_node_string.to_string(),
                            DialogCustomInfos::new(
                                WorldEvent::iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>(),
                                None,
                                WorldEvent::iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>(),
                            ),
                        );

                        let mut current_node = dialog_tree.borrow_mut();

                        // mut?
                        match current_node.content().split_first() {
                            None => {}
                            Some((first, rem)) => {
                                match first {
                                    DialogContent::Text(_) => {
                                        dialog.current_node = if current_node.content().len() > 1 {
                                            // The monologue is not finished
                                            *current_node.content_mut() = rem.to_vec();
                                            Some(current_node.print_file())
                                        } else if current_node.is_end_node() {
                                            info!("clear dialog panel");
                                            None
                                        } else {
                                            // TODO: if verify and there is not one choice verified: don't overwrite the current_node to let the last npc sentence
                                            // DOC: Specifics Rules link - Children (Text/Choice)
                                            Some(
                                                current_node.children()[*child_index]
                                                    .borrow()
                                                    .print_file(),
                                            )
                                        };

                                        if current_node.content().len() <= 1 {
                                            trigger_event.send(TriggerEvents(
                                                current_node.trigger_event().to_vec(),
                                            ));
                                        }
                                    }
                                    DialogContent::Choice {
                                        text: _,
                                        condition: _,
                                    } => {
                                        if *skip {
                                            return;
                                        }
                                        dialog.current_node = if current_node.is_end_node() {
                                            info!("clear dialog panel");
                                            None
                                        } else {
                                            // DOC: Specifics Rules link - Children (Text/Choice)
                                            Some(
                                                current_node.children()[*child_index]
                                                    .borrow()
                                                    .print_file(),
                                            )
                                        };
                                        trigger_event.send(TriggerEvents(
                                            current_node.trigger_event().to_vec(),
                                        ));
                                    }
                                }
                            }
                        };
                    }
                }
            }
        }
    }
}

/// # Purpose
///
/// When the dialog file implied in the talk is changed,
/// updates the panels' content.
///
/// # Process
///
/// check the current node from the interlocutor
/// DOC: TODO
/// - this is a text
///   - change the text from the upper_scroll
///   - clear the player_scroll (choice panel)
/// - this is a choice
///   - Player Choice
///     - update the player_scroll (implied: let the upper_scroll)
///   - NPC Choice
///     TODO: feature - NPC Choice
///     for now, the player has to choose what the npc should say..
fn update_dialog_panel(
    current_interlocutor: Res<CurrentInterlocutor>,
    active_world_events: Res<ActiveWorldEvents>,
    dialog_changed_query: Query<Entity, Changed<Dialog>>,
    dialog_query: Query<&Dialog, With<Portrait>>,

    mut npc_panel_query: Query<&mut Text, (With<NPCPanel>, Without<PlayerPanel>)>,
    mut player_panel_query: Query<&mut Text, (With<PlayerPanel>, Without<NPCPanel>)>,
    mut player_choices_query: Query<(&Choice, &mut Visibility, &Children)>,
    mut text_query: Query<&mut Text, (Without<PlayerPanel>, Without<NPCPanel>)>,

    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
) {
    if !current_interlocutor.is_none()
        && (current_interlocutor.is_changed() || !dialog_changed_query.is_empty())
    {
        // info!("UpdateDialogPanel");
        let dialog = dialog_query
            .get(current_interlocutor.interlocutor.unwrap())
            .unwrap();

        let mut player_text = player_panel_query.single_mut();
        let mut npc_text = npc_panel_query.single_mut();

        match &dialog.current_node {
            None => {
                npc_text.sections[0].value.clear();
                player_text.sections[0].value.clear();
                for (_, mut visibility, _) in &mut player_choices_query {
                    *visibility = Visibility::Hidden;
                }
            }
            Some(current) => {
                let dialog_tree = init_tree_file(
                    current.to_owned(),
                    DialogCustomInfos::new(
                        WorldEvent::iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>(),
                        None,
                        WorldEvent::iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>(),
                    ),
                );

                let current_node = dialog_tree.borrow();
                let dialogs = &current_node.content();

                match &dialogs.first() {
                    // REFACTOR: Stop panic
                    None => panic!("Err: dialog_content is empty"),
                    Some(DialogContent::Text(text)) => {
                        if current_node.author().clone().unwrap() == "Player" {
                            player_text.sections[0].value = text.clone();
                        } else {
                            // replace the entire npc panel's content
                            npc_text.sections[0].value = text.clone();

                            // Clear the previous choice if there is any
                            for (_, mut visibility, _) in &mut player_choices_query {
                                *visibility = Visibility::Hidden;
                            }
                        }
                    }
                    Some(DialogContent::Choice {
                        text: _,
                        condition: _,
                    }) => {
                        if current_node.author().clone().unwrap() == "Player" {
                            // replace current by the new set of choices
                            let mut choices = Vec::<String>::new();
                            for dialog in dialogs.iter() {
                                match dialog {
                                    DialogContent::Choice { text, condition } => {
                                        match condition {
                                            Some(cond) => {
                                                if cond.is_verified(
                                                    None,
                                                    Some(active_world_events.active_world_events
                                                        .iter()
                                                        .map(|x| x.to_string())
                                                        .collect::<Vec<String>>())
                                                ) {
                                                    choices.push(text.to_owned());
                                                    // info!("DEBUG: add choice: {}", text);
                                                }
                                            }
                                            // no condition
                                            None => {
                                                choices.push(text.to_owned());
                                                // info!("DEBUG: add choice: {}", text);
                                            }
                                        }
                                    }
                                    _ => panic!(
                                        "Err: DialogTree Incorrect; A choices' vector contains something else"
                                    ),
                                }
                            }
                            player_text.sections[0].value.clear();

                            for (choice_index, mut visibility, children) in
                                &mut player_choices_query
                            {
                                // Here you could compare the index with `dialogs.len()` to incorpore all choice but
                                // lock the unsatisfied choice's condition
                                if choice_index.0 < choices.len() {
                                    let mut text = text_query.get_mut(children[0]).unwrap();
                                    text.sections[0].value = choices[choice_index.0].clone();
                                    *visibility = Visibility::Inherited;
                                } else {
                                    *visibility = Visibility::Hidden;
                                }
                            }

                            // Remove all text which aren't said by the current interlocutor
                            if current_interlocutor.is_changed() {
                                npc_text.sections[0].value.clear();
                            }
                        } else {
                            // NPC Choices
                            let mut possible_choices_index: Vec<usize> = Vec::new();
                            for (index, dialog) in dialogs.iter().enumerate() {
                                match dialog {
                                    DialogContent::Choice { text: _, condition } => {
                                        match condition {
                                            Some(cond) => {
                                                if cond.is_verified(
                                                    None,
                                                    Some(active_world_events.active_world_events
                                                        .iter()
                                                        .map(|x| x.to_string())
                                                        .collect::<Vec<String>>())
                                                ) {
                                                    possible_choices_index.push(index);
                                                }
                                            }
                                            // no condition
                                            None => {
                                                possible_choices_index.push(index);
                                            }
                                        }
                                    }
                                    _ => panic!(
                                        "Err: DialogTree Incorrect; A choices' vector contains something else"
                                    ),
                                }
                            }
                            if let Some(child_index) =
                                possible_choices_index.choose(&mut rand::thread_rng())
                            {
                                dialog_dive_event.send(DialogDiveEvent {
                                    child_index: *child_index,
                                    skip: false,
                                });
                            } else {
                                // TODO: if `possible_choices_index.is_empty()`
                            }
                        }
                    }
                }
            }
        }
    }
}

fn change_interlocutor_portrait(
    current_interlocutor: Res<CurrentInterlocutor>,
    mut portrait_panel_query: Query<&mut UiImage, With<InterlocutorPortait>>,
    portraits_query: Query<&UiImage, (With<Portrait>, Without<InterlocutorPortait>)>,
    asset_server: Res<AssetServer>,
) {
    if current_interlocutor.is_changed() {
        let mut portrait = portrait_panel_query.single_mut();
        portrait.texture = match current_interlocutor.interlocutor {
            None => asset_server.load("textures/character/background.png"),
            Some(interlocutor) => {
                let new_portrait = portraits_query.get(interlocutor).unwrap();
                new_portrait.texture.clone()
            }
        };
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => *color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

// /// Disables empty button.
// ///
// /// Prevents checking a index in the choices list.
// fn button_visibility(
//     mut choice_buttons_query: Query<(&mut Visibility, &Choice, &Children), With<Button>>,
//     text_changed_query: Query<&Text, Changed<Text>>,
// ) {
//     if !text_changed_query.is_empty() {
//         for (mut visibility, _choice_index, children) in &mut choice_buttons_query {
//             if let Ok(text) = text_changed_query.get(children[0]) {
//                 *visibility = if text.sections.is_empty() {
//                     Visibility::Hidden
//                 } else {
//                     Visibility::Inherited
//                 };
//             }
//         }
//     }
// }

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

pub const OLD_FROG_DIALOG: &str = "# Old Frog

- KeroKero
- I want you to talk with the last Frog.\\nAll the way.

## Player

- Done ? | e: FrogTalk;

### Old Frog

- You have my respect.
- Press Reset or alt+f4.\n";

pub const FROG_DIALOG: &str = "# Frog

- KeroKero | None
- Crôaa | None
- bêêh | None

## Frog

- KeroKero

### Player

- I wanted to say you something

#### Player

- You = Cool | None
- You = Not Cool | None

##### Frog

- Big love on you

-> FrogLove

##### Frog

- I'm sad now.

-> FrogHate

## Frog

- Crôaa

### Player

- I wanted to say you something

#### Player

- You = Cool | None
- You = Not Cool | None

##### Frog

- Big love on you

-> FrogLove

##### Frog

- I'm sad now.

-> FrogHate

## Frog

- Bêêh

### Player

- I wanted to say you something

#### Player

- You = Cool | None
- You = Not Cool | None

##### Frog

- Big love on you

-> FrogLove

##### Frog

- I'm sad now.

-> FrogHate\n";

pub const WARRIOR_DIALOG: &str = "# Warrior Frog

- Hey
- I mean... KeroKero
- Can you bring my love to my homegirl the Frog in the Middle ?

## Player

- Oh Jeez I messed up | e: FrogHate;
- THe Frog is in love | e: FrogLove;

### Warrior Frog

- :0

-> FrogTalk

### Warrior Frog

- :)

-> FrogTalk\n";

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
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Higher Part"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.), Val::Px(65.)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            Name::new(format!("Reset Button")),
                            Reset,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Reset",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::height(Val::Percent(70.)),
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
                                        texture: asset_server
                                            .load("textures/character/Icons_12.png"),
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
                                Portrait,
                                Interaction::default(),
                                Dialog::all(Some(String::from(OLD_FROG_DIALOG))),
                            ));

                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server
                                            .load("textures/character/Icons_23.png"),
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
                                Portrait,
                                Interaction::default(),
                                Dialog::all(Some(String::from(FROG_DIALOG))),
                            ));

                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server
                                            .load("textures/character/Icons_27.png"),
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
                                Interaction::default(),
                                Portrait,
                                Dialog::all(Some(String::from(WARRIOR_DIALOG))),
                            ));
                        });
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
                                InterlocutorPortait,
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
                                            color: Color::WHITE,
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
                                NPCPanel,
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
                                                color: Color::WHITE,
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
                                    PlayerPanel,
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
                                                Name::new(format!("Choice n°{i}")),
                                                Choice(i),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "",
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
