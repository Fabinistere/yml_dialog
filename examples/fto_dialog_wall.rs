use std::{fmt, str::FromStr};

use bevy::{prelude::*,winit::WinitSettings, render::texture::ImagePlugin, window::WindowResolution};
use bevy_tweening::TweeningPlugin;

use constants::{FIXED_TIME_STEP, CLEAR, HEIGHT, RESOLUTION, character::{dialog::FABIEN_DIALOG, KARMA_MIN, KARMA_MAX}};
use fto_dialog::DialogCustomInfos;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    Discussion,
}

#[derive(Component)]
pub struct Karma(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct NPC;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter)]
enum WorldEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
    HasCharisma,
    HasFriend,
}

impl fmt::Display for WorldEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorldEvent::BeatTheGame => write!(f, "BeatTheGame"),
            WorldEvent::FirstKill => write!(f, "FirstKill"),
            WorldEvent::AreaCleared => write!(f, "AreaCleared"),
            WorldEvent::HasCharisma => write!(f, "HasCharisma"),
            WorldEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for WorldEvent {
    type Err = ();

    fn from_str(input: &str) -> Result<WorldEvent, Self::Err> {
        match input {
            "BeatTheGame" => Ok(WorldEvent::BeatTheGame),
            "FirstKill" => Ok(WorldEvent::FirstKill),
            "AreaCleared" => Ok(WorldEvent::AreaCleared),
            "HasCharisma" => Ok(WorldEvent::HasCharisma),
            "HasFriend" => Ok(WorldEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// List all triggerable event,
/// that can be send when quitting a dialog node
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter)]
enum TriggerEvent {
    FightEvent,
    HasFriend,
}
impl fmt::Display for TriggerEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TriggerEvent::FightEvent => write!(f, "FightEvent"),
            TriggerEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for TriggerEvent {
    type Err = ();

    fn from_str(input: &str) -> Result<TriggerEvent, Self::Err> {
        match input {
            "FightEvent" => Ok(TriggerEvent::FightEvent),
            "HasFriend" => Ok(TriggerEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// Contains the current Dialog node of the interlocutor's talk.
///
/// Holds a String which can be converted to a `Rc<RefCell<DialogNode>>`
/// by `print_file()`
#[derive(
    Deref, DerefMut, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Component,
)]
pub struct Dialog {
    current_node: Option<String>,
}

impl Dialog {
    /// Constructs a new Dialog from the given `str`
    ///
    /// # Node
    ///
    /// TODO: feature - Read at dialog_file
    pub fn from_str(str: &str) -> Dialog {
        Dialog {
            current_node: Some(str.to_string()),
        }
    }

    /// Constructs an empty Dialog `None`
    pub fn new() -> Dialog {
        Dialog::default()
    }

    /// Returns the read-only current node (an `&Option<String>`)
    pub fn current_node(&self) -> &Option<String> {
        &self.current_node
    }

    /// Returns the mutable reference current node (an `&Option<String>`)
    pub fn current_node_mut(&mut self) -> &mut Option<String> {
        &mut self.current_node
    }
}

/// Hold the list of `WorldEvent`, `TriggerEvent` and the karma min/max
#[derive(Deref, DerefMut, Resource)]
pub struct OurInfos(DialogCustomInfos);

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
        /* -------------------------------------------------------------------------- */
        /*                                  Our Code                                  */
        /* -------------------------------------------------------------------------- */
        .insert_resource(OurInfos(DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        )))
        .add_plugin(UiPlugin)
        .add_startup_systems((spawn_camera, spawn_player));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Karma(50), Dialog::from_str(FABIEN_DIALOG), Name::new("Player")));
}


#[derive(Component)]
pub struct UiElement;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())

            .add_event::<dialog_panel::CreateDialogPanelEvent>()
            .add_event::<dialog_panel::CloseDialogPanelEvent>()
            .add_event::<dialog_panel::EndNodeDialogEvent>()
            .add_event::<dialog_scroll::UpdateScrollEvent>()
            .add_event::<dialog_player::DialogDiveEvent>()
            .add_event::<dialog_player::DropFirstTextUpperScroll>()
            .add_event::<dialog_box::ResetDialogBoxEvent>()
            // Trigger Event
            // .add_event::<dialog_system::FightEvent>()
            // .add_event::<dialog_system::TriggerEvent>()

            .add_startup_system(dialog_panel::load_textures)

            // OPTIMIZE: System Ordering

            .add_systems((
                dialog_panel::create_dialog_panel_on_key_press,
                dialog_panel::create_dialog_panel,

                dialog_panel::update_dialog_panel,
                dialog_panel::update_dialog_tree,

                dialog_scroll::animate_scroll,

                dialog_scroll::update_upper_scroll,
                dialog_scroll::update_player_scroll,

                dialog_box::reset_dialog_box,
                dialog_box::update_dialog_box,

                dialog_player::button_system,
                dialog_player::hide_empty_button,
                dialog_player::skip_forward_dialog,

                dialog_player::dialog_dive,
                dialog_player::drop_first_text_upper_scroll,
                // crash when in this big tuple: (but not when in a simple `.add_system()`)
                // dialog_player::throw_trigger_event.after(dialog_player::dialog_dive),
            ))
            
            // crash when in this big tuple: (but not when in a simple `.add_system()`)
            .add_system(dialog_panel::end_node_dialog)
            .add_system(dialog_panel::close_dialog_panel)
            .add_system(dialog_panel::despawn_dialog_panel)
            ;
    }
}

mod dialog_box {
    //! Every Structs and methods only about Dialog Box

    use bevy::prelude::*;

    use crate::{dialog_scroll::{PlayerChoice, UpperScroll}, constants::ui::dialogs::DIALOG_BOX_UPDATE_DELTA_S};

    /// Represents the entity containing the displayed text as first children.
    ///
    /// Used to animate the Text, letter by letter.
    #[derive(Debug, Component)]
    pub struct DialogBox {
        pub text: String,
        progress: usize,
        finished: bool,
        update_timer: Timer,
    }

    impl DialogBox {
        pub fn new(text: String, update_time: f32) -> Self {
            DialogBox {
                text,
                progress: 0,
                finished: false,
                update_timer: Timer::from_seconds(update_time, TimerMode::Once),
            }
        }

        // /// Same as new but keep the signature
        // fn reset(&self, text: String, update_time: f32) {
        //     *self.text = text;
        //     *self.progress = 0;
        //     *self.finished = false;
        //     *self.update_timer = Timer::from_seconds(update_time, TimerMode::Once);
        // }
    }

    /// Happens when
    ///   - ui::dialog_panel::update_upper_scroll
    ///     - updates UpperScroll Text with the UpperScroll infos
    ///   - ui::dialog_panel::update_player_scroll
    ///     - updates PlayerScroll Text with the UpperScroll infos
    ///     happens for every choice there is in the PlayerScroll
    ///
    /// Read in
    ///   - ui::dialog_panel::reset_dialog_box
    ///     - creates a DialogBox to transfer info to the child Text
    ///     if there is none
    ///     or resets the text and dialogBox
    pub struct ResetDialogBoxEvent {
        pub dialog_box: Entity,
        /// could be
        ///
        /// - a Choice
        /// - a Text
        pub text: String,
    }

    /// Animates, letter by letter, each Text.
    /// ( being the DialogBox's 1rt child )
    pub fn update_dialog_box(
        time: Res<Time>,
        mut dialog_box_query: Query<(&mut DialogBox, &Children)>,
        mut text_query: Query<&mut Text>,
    ) {
        for (mut dialog_box, children) in dialog_box_query.iter_mut() {
            dialog_box.update_timer.tick(time.delta());

            if dialog_box.update_timer.finished() && !dialog_box.finished {
                // let mut text = text_query.get_mut(children[0]).unwrap();
                match text_query.get_mut(children[0]) {
                    Ok(mut text) => {
                        // prompt the simple text
                        // FIXME: bug - if the given text contains a accent this will crash
                        match dialog_box.text.chars().nth(dialog_box.progress) {
                            // will ignore any louche symbol
                            // FIXME: infinite call when there is a accent
                            None => warn!("Accent Typical Crash"),
                            Some(next_letter) => {
                                text.sections[0].value.push(next_letter);

                                dialog_box.progress += 1;
                                if dialog_box.progress >= dialog_box.text.len() {
                                    dialog_box.finished = true;
                                }
                            }
                        }
                    }
                    // FIXME: If there is no TEXT then insert one in it
                    // pb: on which scroll...
                    Err(e) => warn!("No Text in the Dialog Wall: {:?}", e),
                }
            }
        }
    }

    /// Reset DialogBox on Event
    pub fn reset_dialog_box(
        mut commands: Commands,

        mut reset_event: EventReader<ResetDialogBoxEvent>,

        mut dialog_box_query: Query<
            (Entity, &mut DialogBox, &Children),
            Or<(With<PlayerChoice>, With<UpperScroll>)>,
        >,
        mut text_query: Query<&mut Text>,
    ) {
        for ResetDialogBoxEvent { dialog_box, text } in reset_event.iter() {
            match dialog_box_query.get_mut(*dialog_box) {
                Err(_e) => {
                    info!("DEBUG: no DialogBox in the UpperScroll");
                    commands
                        .entity(*dialog_box)
                        .insert(DialogBox::new(text.to_string(), DIALOG_BOX_UPDATE_DELTA_S));
                }
                Ok((_, mut dialog_box, children)) => {
                    // FIXME: bug - Reset the text even if there is no change
                    // Clear the DialogBox Child: the Text
                    match text_query.get_mut(children[0]) {
                        Err(e) => warn!("No Text Section: {:?}", e),
                        Ok(mut dialog_text) => {
                            if dialog_box.text != *text {
                                dialog_text.sections[0].value.clear();
                                // replace current DialogBox with a brand new one
                                *dialog_box =
                                    DialogBox::new(text.to_string(), DIALOG_BOX_UPDATE_DELTA_S);
                            }
                        }
                    }
                }
            }
        }
    }
}

mod dialog_panel {
    //! All base method involved in creating the UI ingame
    //!
    //! EventHandler:
    //!
    //! - Enter in Combat
    //! - Exit in Combat
    //! - Open HUD manually (pressing 'o')
    //! - Scolls Gestion
    //!   - Update Dialog Tree
    //!   - Update each Scroll
    //!   - Update Dialog Box / Text

    use bevy::prelude::*;
    use bevy_tweening::{lens::UiPositionLens, *};
    use std::time::Duration;

    use fto_dialog::{init_tree_file, DialogContent};

    use crate::{
        dialog_scroll::{
            PlayerChoice, PlayerScroll, Scroll, ScrollTimer, UpdateScrollEvent, UpperScroll,
        },
        constants::ui::dialogs::{
            DIALOG_PANEL_ANIMATION_TIME_MS, DIALOG_PANEL_ANIMATION_OFFSET, SCROLL_ANIMATION_DELTA_S, NORMAL_BUTTON, SCROLL_ANIMATION_FRAMES_NUMBER
        },
        Karma, Player, NPC, Dialog, OurInfos,
    };

    /// Represents The UI Wall.
    /// Every UI Wall is associated with an entity (can be the player or the interlocutor like an object).
    ///
    /// If the dialog_tree changes, it will update the scrolls.
    ///
    /// To modify the dialog, just modify the DialogPanel.
    ///
    /// # Note
    ///
    /// TODO: feature - sync author with interlocutor (to know which one is talking)
    /// REFACTOR: Turn DialogPanel into a Resource
    ///
    #[derive(Component, Reflect)]
    pub struct DialogPanel {
        // keep track of the origanal interlocutor
        // their dialog will be change/update in update_dialog_tree
        pub main_interlocutor: Entity,
        // XXX: will allow us to detect change especially in the opening
        pub dialog_tree: String,
    }

    /// Happens when
    ///   - ui::dialog_panel::create_dialog_panel_on_key_press
    ///     - press 'o' to open the UI
    ///   - ui::dialog_panel::create_dialog_panel_on_combat_event
    ///     - when CombatEvent is triggered ( and ui not open )
    ///     open the ui with the interlocutor gived by the CombatEvent
    /// Read in
    ///   - ui::dialog_panel::create_dialog_panel
    ///     - for a given String, creates basic ui entities ( ui + fx )
    pub struct CreateDialogPanelEvent {
        interlocutor: Entity,
        dialog_tree: String,
    }

    /// Happens when
    ///   - ui::dialog_panel::update_dialog_panel
    ///     - the dialog tree contained within the DialogPanel is empty
    ///
    /// Read in
    ///   - ui::dialog_panel::end_node_dialog
    ///     - fills the given interlocutor with a blank "..." dialog
    ///     and exit the Combat ( send CombatExitEvent )
    pub struct EndNodeDialogEvent;

    /// Happens when
    ///   - ui::dialog_panel::create_dialog_panel_on_key_press
    ///     - ui already open
    ///   - ui::dialog_panel::create_dialog_panel_on_combat_event
    ///     - ui already open
    ///   - combat::mod::exit_combat
    ///     - Close the Ui when CombatExitevent triggered
    ///
    /// Read in
    ///   - ui::dialog_panel::close_dialog_panel
    ///     - close ui
    pub struct CloseDialogPanelEvent;

    #[derive(Resource)]
    pub struct DialogPanelResources {
        text_font: Handle<Font>,
        appartements: Handle<Image>,
        stained_glass_panels: Handle<Image>,
        background: Handle<Image>,
        stained_glass_opened: Handle<Image>,
        chandelier: Handle<Image>,
        pub scroll_animation: Vec<Handle<Image>>,
    }

    pub fn load_textures(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        // let scroll_texture = asset_server.load("textures/hud/scroll_animation.png");
        // let scroll_atlas = TextureAtlas::from_grid(scroll_texture, SCROLL_SIZE.into(), 1, 45);

        let mut scroll_animation_frames = vec![];
        for i in 0..SCROLL_ANIMATION_FRAMES_NUMBER {
            scroll_animation_frames
                .push(asset_server.load(&format!("textures/hud/scroll_animation/frame_{}.png", i)));
        }

        commands.insert_resource(DialogPanelResources {
            text_font: asset_server.load("fonts/dpcomic.ttf"),
            appartements: asset_server.load("textures/hud/papier_paint.png"),
            background: asset_server.load("textures/hud/dialog_background.png"),
            scroll_animation: scroll_animation_frames,
            chandelier: asset_server.load("textures/hud/chandelier.png"),
            stained_glass_opened: asset_server.load("textures/hud/stained_glass_opened.png"),
            stained_glass_panels: asset_server.load("textures/hud/stained_glass_panels.png"),
        });
    }

    /// Open the gialog panel on any input
    ///
    /// # Behavior
    ///
    /// Interpret the dialog carried by the entity.
    ///
    /// In Dialog Sequence,
    /// we might -want to- have the last text
    /// when the player is ask to choose a answer.
    ///
    /// For simplificity,
    /// the feature: `recreate the dialog tree to include the last text in the root`
    /// is deactivated.
    ///
    /// So, when the dialog is stopped during a choice,
    /// the root of the dialog tree is not modified and contains only the previous choice.
    ///
    /// Unlucky situation :
    /// having to answer something without the context.
    ///
    /// # Note
    ///
    ///
    /// FIXME: PB Spamming the ui key 'o'; ?throws an error
    pub fn create_dialog_panel_on_key_press(
        mut create_dialog_panel_event: EventWriter<CreateDialogPanelEvent>,
        mut close_dialog_panel_event: EventWriter<CloseDialogPanelEvent>,

        query: Query<(Entity, &Animator<Style>, &Style), With<DialogPanel>>,
        keyboard_input: Res<Input<KeyCode>>,
        // mut key_evr: EventReader<KeyboardInput>,
        player_query: Query<(Entity, &Dialog), With<Player>>,
    ) {
        // REFACTOR: check [ScanCode](https://bevy-cheatbook.github.io/input/keyboard.html#key-codes-and-scan-codes) instead
        if keyboard_input.just_pressed(KeyCode::O) {
            if let Ok((_entity, animator, _style)) = query.get_single() {
                if animator.tweenable().progress() >= 1.0 {
                    close_dialog_panel_event.send(CloseDialogPanelEvent);
                }
            } else {
                // keep track of player's personal thoughts
                let (player, dialog) = player_query.single();

                let dialog_tree: String = match &dialog.current_node() {
                    Some(text) => text.to_owned(),
                    None => String::new(),
                };

                create_dialog_panel_event.send(CreateDialogPanelEvent {
                    interlocutor: player,
                    dialog_tree,
                });
            }
        }

        // REFACTOR: Will work with: only running in a custom State
        // for ev in key_evr.iter() {
        //     if ev.state == ButtonState::Pressed {
        //         if let Ok((_entity, animator, _style)) = query.get_single() {
        //             if animator.tweenable().progress() >= 1.0 {
        //                 close_dialog_panel_event.send(CloseDialogPanelEvent);
        //             }
        //         } else {
        //             // keep track of player's personal thoughts
        //             let (player, dialog) = player_query.single();

        //             let dialog_tree: String;
        //             match &dialog.current_node {
        //                 Some(text) => dialog_tree = text.to_owned(),
        //                 None => dialog_tree = String::new(),
        //             }

        //             create_dialog_panel_event.send(CreateDialogPanelEvent {
        //                 interlocutor: player,
        //                 dialog_tree,
        //             });
        //         }
        //     }
        // }
    }

    pub fn close_dialog_panel(
        mut commands: Commands,
        mut close_dialog_panel_events: EventReader<CloseDialogPanelEvent>,
        mut query: Query<(Entity, &mut Animator<Style>, &Style), With<DialogPanel>>,
    ) {
        for CloseDialogPanelEvent in close_dialog_panel_events.iter() {
            info!("close dialog event");
            if let Ok((entity, mut _animator, style)) = query.get_single_mut() {
                let dialog_panel_tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(DIALOG_PANEL_ANIMATION_TIME_MS),
                    UiPositionLens {
                        start: style.position,
                        end: UiRect {
                            left: Val::Auto,
                            top: Val::Px(0.0),
                            right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                            bottom: Val::Px(0.0),
                        },
                    },
                )
                .with_completed_event(0);

                commands
                    .entity(entity)
                    .remove::<Animator<Style>>()
                    .insert(Animator::new(dialog_panel_tween));
            }
        }
    }

    pub fn despawn_dialog_panel(
        mut commands: Commands,
        mut completed_event: EventReader<TweenCompleted>,
    ) {
        for TweenCompleted { entity, user_data } in completed_event.iter() {
            if *user_data == 0 {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }

    pub fn create_dialog_panel(
        mut create_dialog_panel_events: EventReader<CreateDialogPanelEvent>,
        mut create_scroll_content: EventWriter<UpdateScrollEvent>,

        mut commands: Commands,
        mut _meshes: ResMut<Assets<Mesh>>,
        _texture_atlases: Res<Assets<TextureAtlas>>,
        dialog_panel_resources: Res<DialogPanelResources>,
        asset_server: Res<AssetServer>,
    ) {
        for CreateDialogPanelEvent {
            interlocutor,
            dialog_tree,
        } in create_dialog_panel_events.iter()
        {
            info!("open dialog event");

            let dialog_panel_tween = Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_millis(DIALOG_PANEL_ANIMATION_TIME_MS),
                UiPositionLens {
                    start: UiRect {
                        left: Val::Auto,
                        top: Val::Px(0.0),
                        right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                        bottom: Val::Px(0.0),
                    },
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                },
            );

            let panels_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_millis(1000),
                UiPositionLens {
                    start: UiRect {
                        top: Val::Px(0.0),
                        ..UiRect::default()
                    },
                    end: UiRect {
                        top: Val::Px(-160.0),
                        ..UiRect::default()
                    },
                },
            );

            commands
                .spawn((
                    // We spawn the paper wall background.
                    // To hide the windows' panels when reaching
                    // the top of the window.
                    // Because the main Wall Background is above these panels.
                    ImageBundle {
                        image: dialog_panel_resources.appartements.clone().into(),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Relative,
                            position: UiRect {
                                top: Val::Px(0.0),
                                left: Val::Auto,
                                right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                                bottom: Val::Px(0.0),
                            },
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            size: Size::new(Val::Auto, Val::Percent(100.0)),
                            aspect_ratio: Some(284.0 / 400.0),
                            ..default()
                        },
                        ..default()
                    },
                    // REFACTOR: Turn DialogPanel into a Resource
                    DialogPanel {
                        main_interlocutor: *interlocutor,
                        dialog_tree: dialog_tree.to_owned(),
                    },
                    Animator::new(dialog_panel_tween),
                    Name::new("UI Wall"),
                ))
                .with_children(|parent| {
                    let child_sprite_style = Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        ..default()
                    };

                    // panels under the wall to prevent them from sticking out of the window after being lifted.
                    parent.spawn((
                        ImageBundle {
                            image: dialog_panel_resources.stained_glass_panels.clone().into(),
                            style: child_sprite_style.clone(),
                            ..default()
                        },
                        Animator::new(panels_tween),
                        Name::new("Stained Glass Panel"),
                    ));

                    parent.spawn((
                        ImageBundle {
                            image: dialog_panel_resources.background.clone().into(),
                            style: child_sprite_style.clone(),
                            ..default()
                        },
                        Name::new("Wall Background"),
                    ));

                    parent.spawn((
                        ImageBundle {
                            image: dialog_panel_resources.stained_glass_opened.clone().into(),
                            style: child_sprite_style.clone(),
                            ..default()
                        },
                        Name::new("Stained Glass Static"),
                    ));

                    parent.spawn((
                        ImageBundle {
                            image: dialog_panel_resources.chandelier.clone().into(),
                            style: child_sprite_style,
                            ..default()
                        },
                        Name::new("Light"),
                    ));

                    // Upper Scroll

                    parent
                    .spawn((
                        ImageBundle {
                            image: dialog_panel_resources.scroll_animation[0].clone().into(),
                            style: Style {
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::FlexEnd,
                                ..default()
                            },
                            ..default()
                        },
                        Scroll {
                            current_frame: 0,
                            reverse: false,
                        },
                        UpperScroll {
                            // will be changed in update_dialog_panel
                            texts: vec![],
                        },
                        ScrollTimer(Timer::from_seconds(
                            SCROLL_ANIMATION_DELTA_S,
                            TimerMode::Once,
                        )),
                        Name::new("Upper Scroll"),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: dialog_panel_resources.text_font.clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_alignment(TextAlignment::Left),
                            style: Style {
                                flex_wrap: FlexWrap::Wrap,
                                position: UiRect {
                                    top: Val::Px(375.0),
                                    ..UiRect::default()
                                },
                                margin: UiRect {
                                    left: Val::Percent(24.0),
                                    ..UiRect::default()
                                },
                                size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                                ..default()
                            },
                            ..default()
                        });
                    })
                    // .insert(DialogBox::new(dialog[0].clone(), DIALOG_BOX_UPDATE_DELTA_S))
                    ;

                    // parent
                    //     .spawn(ImageBundle {
                    //         image: texture_atlases
                    //             .get(dialog_panel_resources.scroll_animation.clone())
                    //             .unwrap()
                    //             .texture
                    //             .clone_weak()
                    //             .into(),
                    //         style: child_sprite_style.clone(),
                    //         ..default()
                    //     });

                    // Player Scroll

                    let player_scroll_img =
                        asset_server.load("textures/hud/HUD_1px_parchemin_MC_ouvert.png");

                    parent
                        .spawn((
                            ImageBundle {
                                image: player_scroll_img.into(),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::FlexEnd,
                                    ..default()
                                },
                                ..default()
                            },
                            Scroll {
                                current_frame: 0,
                                reverse: false,
                            },
                            PlayerScroll {
                                // will be changed in update_dialog_panel
                                choices: vec![],
                            },
                            ScrollTimer(Timer::from_seconds(
                                SCROLL_ANIMATION_DELTA_S,
                                TimerMode::Once,
                            )),
                            Name::new("Player Scroll"),
                        ))
                        .with_children(|parent| {
                            // TODO: feature - 3 PlayerChoice is enough, to have much reuse theses three in another page

                            // First potential choice
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            // TODO: custom size ? (text dependent)
                                            size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                            margin: UiRect::all(Val::Auto),
                                            // margin: UiRect {
                                            //     top: Val::Percent(105.0),
                                            //     left: Val::Percent(24.0),
                                            //     ..UiRect::default()
                                            // },
                                            // justify_content: JustifyContent::SpaceAround,
                                            position: UiRect {
                                                top: Val::Px(450.0),
                                                left: Val::Px(10.0),
                                                ..UiRect::default()
                                            },
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    PlayerChoice(0),
                                    Name::new("First Choice"),
                                ))
                                // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font: dialog_panel_resources.text_font.clone(),
                                                // TODO: Find the correct value for the choice font size
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Left),
                                        style: Style {
                                            flex_wrap: FlexWrap::Wrap,
                                            margin: UiRect {
                                                // top: Val::Percent(10.0),
                                                ..UiRect::default()
                                            },
                                            max_size: Size::new(
                                                Val::Px(300.0),
                                                Val::Percent(100.0),
                                            ),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });

                            // Second potential choice
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            // TODO: custom size ? (text dependent)
                                            size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                            position: UiRect {
                                                top: Val::Px(250.0),
                                                left: Val::Px(10.0),
                                                ..UiRect::default()
                                            },
                                            margin: UiRect::all(Val::Auto),
                                            // margin: UiRect {
                                            //     top: Val::Percent(125.0),
                                            //     left: Val::Percent(24.0),
                                            //     ..UiRect::default()
                                            // },
                                            // justify_content: JustifyContent::SpaceAround,
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    PlayerChoice(1),
                                    Name::new("Second Choice"),
                                ))
                                // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font: dialog_panel_resources.text_font.clone(),
                                                // TODO: Find the correct value for the choice font size
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Left),
                                        style: Style {
                                            flex_wrap: FlexWrap::Wrap,
                                            margin: UiRect {
                                                // top: Val::Percent(10.0),
                                                ..UiRect::default()
                                            },
                                            max_size: Size::new(
                                                Val::Px(300.0),
                                                Val::Percent(100.0),
                                            ),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });

                            // Third potential choice
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            // TODO: custom size ? (text dependent)
                                            size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                            position: UiRect {
                                                top: Val::Px(50.0),
                                                left: Val::Px(10.0),
                                                ..UiRect::default()
                                            },
                                            margin: UiRect::all(Val::Auto),
                                            // margin: UiRect {
                                            //     top: Val::Percent(145.0),
                                            //     left: Val::Percent(24.0),
                                            //     ..UiRect::default()
                                            // },
                                            // justify_content: JustifyContent::SpaceAround,
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    PlayerChoice(2),
                                    Name::new("Third Choice"),
                                ))
                                // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font: dialog_panel_resources.text_font.clone(),
                                                // TODO: Find the correct value for the choice font size
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Left),
                                        style: Style {
                                            flex_wrap: FlexWrap::Wrap,
                                            margin: UiRect {
                                                // top: Val::Percent(10.0),
                                                ..UiRect::default()
                                            },
                                            max_size: Size::new(
                                                Val::Px(300.0),
                                                Val::Percent(100.0),
                                            ),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        });

                    // Button

                    // parent
                    //     .spawn(ButtonBundle {
                    //         style: Style {
                    //             size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    //             // center button
                    //             margin: UiRect::all(Val::Auto),
                    //             // horizontally center child text
                    //             justify_content: JustifyContent::Center,
                    //             // vertically center child text
                    //             align_items: AlignItems::Center,
                    //             ..default()
                    //         },
                    //         background_color: NORMAL_BUTTON.into(),
                    //         ..default()
                    //     })
                    //     .with_children(|parent| {
                    //         parent.spawn(TextBundle::from_section(
                    //             "Button",
                    //             TextStyle {
                    //                 font: asset_server.load("fonts/dpcomic.ttf"),
                    //                 font_size: 40.0,
                    //                 color: Color::rgb(0.9, 0.9, 0.9),
                    //             },
                    //         ));
                    //     });
                });

            // TOTEST: check with system ordering if this event will be catch
            create_scroll_content.send(UpdateScrollEvent);
        }
    }

    /// # Purpose
    ///
    /// When the dialog file implied in the talk is changed,
    /// updates the scrolls' content.
    ///
    /// # Process
    ///
    /// check the current node from the interlocutor
    ///
    /// - this is a text
    ///   - change the text from the upper_scroll
    ///   - clear the player_scroll (choice panel)
    /// - this is a choice
    ///   - Player Choice
    ///     - update the player_scroll (implied: let the upper_scroll)
    ///   - NPC Choice
    ///     TODO: feature - NPC Choice
    ///     for now, the player has to choose what the npc should say..
    pub fn update_dialog_panel(
        our_infos: Res<OurInfos>,
        panel_query: Query<
            (Entity, &DialogPanel),
            // REFACTOR: Handle the interlocutor change in the UIPanel
            // even detect interlocutor change
            (Changed<DialogPanel>, With<Animator<Style>>),
        >,

        mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,
        mut player_scroll_query: Query<(&mut PlayerScroll, Entity), With<Scroll>>,

        player_query: Query<(Entity, &Karma), With<Player>>,

        mut end_node_dialog_event: EventWriter<EndNodeDialogEvent>,
        mut update_scroll_content: EventWriter<UpdateScrollEvent>,
    ) {
        // REFACTOR: Never Nester Mode requested
        // DOC: Noisy Comments

        // the panel must be open already and their dialog_tree modified
        // else:
        //   just wait for the DialogTree to change;
        //   Nothing change yet
        if let Ok((_ui_wall, panel)) = panel_query.get_single() {
            // info!("DEBUG: smth changed...");

            let dialog_panel = &panel.dialog_tree;
            // println!("DEBUG: \n{:?}", dialog_panel);

            // check what is the current dialog node
            if dialog_panel.is_empty() {
                // info!("DEBUG Empty Dialog Tree");
                end_node_dialog_event.send(EndNodeDialogEvent);
            } else {
                let dialog_tree = init_tree_file(dialog_panel.to_owned(), our_infos.clone());

                let current = &dialog_tree.borrow();

                let dialogs = &current.dialog_content;

                let (mut player_scroll, _player_scroll_entity) = player_scroll_query.single_mut();

                // check the first elem of the DialogContent's Vector
                match &dialogs.first() {
                    None => {
                        // FIXME: handle this err
                        panic!("Err: dialog_content is empty");
                    }
                    Some(DialogContent::Text(_)) => {
                        let mut texts = Vec::<String>::new();
                        for dialog in dialogs.iter() {
                            match dialog {
                            DialogContent::Text(text) => {
                                texts.push(text.to_owned());
                                info!("DEBUG: add text: {}", text);
                            }
                            _ => panic!("Err: DialogTree Incorrect; A texts' vector contains something else"),
                        }
                        }
                        // replace the entire upper scroll's content
                        // FIXME: ¿solved? single - if let - first opening or already open
                        let (mut upper_scroll, _upper_scroll_entity) =
                            upper_scroll_query.single_mut();
                        upper_scroll.texts = texts;

                        // Clear the previous choice if there is any
                        player_scroll.choices.clear();
                    }
                    Some(DialogContent::Choice {
                        text: _,
                        condition: _,
                    }) => {
                        // replace current by the new set of choices
                        let mut choices = Vec::<String>::new();
                        for dialog in dialogs.iter() {
                            match dialog {
                            DialogContent::Choice { text, condition } => {
                                match condition {
                                    Some(cond) => {
                                        let (_player, karma) = player_query.single();
                                        if cond.is_verified(karma.0) {
                                            choices.push(text.to_owned());
                                            info!("DEBUG: add choice: {}", text);
                                        }
                                    }
                                    // no condition
                                    None => {
                                        choices.push(text.to_owned());
                                        info!("DEBUG: add choice: {}", text);
                                    }
                                }
                            }
                            _ => panic!("Err: DialogTree Incorrect; A choices' vector contains something else"),
                        }
                        }
                        // update the player_scroll
                        player_scroll.choices = choices;
                    }
                }
                // ask to update the content of scroll
                update_scroll_content.send(UpdateScrollEvent);
            }
        }
    }

    /// # Save principe
    ///
    /// Updates the String within the entity interlocutor.
    /// This just updates the Dialog contained in the interlocutor to be retrieve the next time we talk with it.
    /// We want to save the dialog progress at each state;
    /// Each time the dialog_tree of the panel is changed
    /// (?OR can be delay to the end of fight)
    ///
    /// # Note
    ///
    /// XXX: little trick to detect change especially in the creation phase
    pub fn update_dialog_tree(
        // XXX: (not an issue.) will detect change if the interlocutor is switch
        dialog_panel_query: Query<&DialogPanel, Changed<DialogPanel>>,
        mut interlocutor_query: Query<(Entity, &mut Dialog)>,
    ) {
        for panel in dialog_panel_query.iter() {
            let interlocutor = panel.main_interlocutor;
            let new_dialog_tree = panel.dialog_tree.clone();
            let (_entity, mut dialog) = interlocutor_query.get_mut(interlocutor).unwrap();
            *dialog.current_node_mut() = Some(new_dialog_tree);
        }
    }

    /// Fills the given interlocutor with a blank "..." dialog
    /// and exit the Combat ( send CombatExitEvent )
    pub fn end_node_dialog(
        mut end_node_dialog_event: EventReader<EndNodeDialogEvent>,

        panel_query: Query<(Entity, &DialogPanel), With<Animator<Style>>>,
        mut interlocutor_query: Query<
            (Entity, &Name, &mut Dialog),
            // Player or NPC
            // TODO: feature - include Object (rm these Withs)
            Or<(With<Player>, With<NPC>)>,
        >,

        mut close_dialog_panel_events: EventWriter<CloseDialogPanelEvent>,
    ) {
        for EndNodeDialogEvent in end_node_dialog_event.iter() {
            info!("DEBUG: EndNodeEvent...");

            let (_ui_wall, panel) = panel_query.single();

            // TODO: feature - manage a cast of NPC choice for each dialog
            // with a priority system to choose
            // engaging a dialog will then choose a certain dialog from the cast
            // leaving mid course will save the current dialog
            // UNLESS there is a overide
            // in case of big event, cancel previous dialog to stick to the main line

            // reset the dialog to the first node: NPC's Choice cast

            info!("exit dialog");

            let interlocutor = panel.main_interlocutor;
            let (_interlocutor_entity, name, mut dialog) =
                interlocutor_query.get_mut(interlocutor).unwrap();

            // replace the current tree by a simple text: `...`
            let display_name = name.replace("NPC ", "");

            // let blank_dialog_tree = "# name\n\n- ...\n"
            //     .replace("name", &display_name)
            //     .to_owned();

            let blank_dialog_tree = format!("# {}\n\n- ...\n", &display_name);

            // don't change panel.dialog_tree here
            // it will be detect by update_dialog_panel
            // i'm living in the fear
            // i'm in danger
            // my own program wants me dead

            // let's overide update_dialog_tree, here and now.
            *dialog.current_node_mut() = Some(blank_dialog_tree);

            // TODO: Close the Dialog
            close_dialog_panel_events.send(CloseDialogPanelEvent);

            // at the next enconter there will be ... as dialog
            // prevent closing the dialog_panel instant after engaging dialog
        }
    }
}

mod dialog_player {
    //! All dialog method handler related with the player directly (input, etc)

    use bevy::prelude::*;
    use bevy_tweening::Animator;

    use fto_dialog::init_tree_file;

    use crate::{
        dialog_panel::DialogPanel,
        dialog_scroll::{PlayerChoice, PlayerScroll, Scroll, UpdateScrollEvent, UpperScroll},
        constants::ui::dialogs::{PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON}, OurInfos
    };

    /// Happens when
    ///   - ui::dialog_player::button_system
    ///     - Choice selected
    ///   - ui::dialog_player::skip_forward_dialog
    ///     - P pressed
    ///
    /// Read in
    ///   - ui::dialog_player::dialog_dive
    ///     - analyze the current node;
    ///     If not empty,
    ///       - drop until there is 1 or less text in the UpeerScroll
    ///       OR
    ///       - go down to the correct child index
    pub struct DialogDiveEvent {
        pub child_index: usize,
        pub skip: bool,
    }

    /// Happens when
    ///   - ui::dialog_player::dialog_dive
    ///     - there is 2 or more text in the UpeerScroll
    /// Read in
    ///   - ui::dialog_player::drop_first_text_upper_scroll
    ///     - drop first text from the UpperScroll
    pub struct DropFirstTextUpperScroll;

    /// Action for each Interaction of the button
    pub fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &PlayerChoice, &Children),
            (Changed<Interaction>, With<Button>),
        >,

        mut dialog_dive_event: EventWriter<DialogDiveEvent>,
        // mut text_query: Query<&mut Text>,
    ) {
        for (interaction, mut color, index, _children) in &mut interaction_query {
            // let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Clicked => {
                    dialog_dive_event.send(DialogDiveEvent {
                        child_index: index.0,
                        skip: false,
                    });

                    // text.sections[0].value = "Press".to_string();
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    // text.sections[0].value = "Hover".to_string();
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    // text.sections[0].value = "Button".to_string();
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    /// When P's pressed, dive into the dialog ( to the first very child )
    ///
    /// # Process
    ///
    /// check the upper scroll content
    ///
    /// Only skip text
    ///
    /// - if len > 1
    ///   - pop the first elem from the vector texts
    /// - else (if) only one text remain or none
    ///   - replace the current dialog node of the panel by its child (of the current)
    pub fn skip_forward_dialog(
        query: Query<(Entity, &Animator<Style>), With<DialogPanel>>,
        keyboard_input: Res<Input<KeyCode>>,

        mut dialog_dive_event: EventWriter<DialogDiveEvent>,
    ) {
        // TODO: feature - if the text is not entirely prompted, skip the animation (update_dialog_box)
        // instead of skiping the text

        // REFACTOR: ? If Ui is open ? instead of testing this query ?
        // or just with ui_wall.finished: bool
        if let Ok((_ui_wall, animator)) = query.get_single() {
            // prevent skip while opening the panel
            if keyboard_input.just_pressed(KeyCode::P) && animator.tweenable().progress() < 1.0 {
                // be patient for god sake
                warn!("attempt of skip while the panel was opening");
                // TODO: feature - skip the animation ?! (i think it's already fast, so no)
            }
            // let any = keyboard_input.any_just_pressed([0, 162]);
            // !keyboard_input.get_just_pressed().is_empty()
            else if keyboard_input.just_pressed(KeyCode::P) {
                info!("DEBUG: P pressed");

                dialog_dive_event.send(DialogDiveEvent {
                    child_index: 0,
                    skip: true,
                });
            }
        }
        // FIXME: prevent more than one Ui Wall open at the same time
    }

    /// Analyze the current node;
    ///
    /// If not empty,
    /// - drop until there is 1 or less text in the UpeerScroll
    /// - go down to the correct child index
    ///
    /// # Note
    ///
    /// Every modification of the DialogPanel's content
    /// will modify the dialog contained the concerned interlocutor
    ///
    /// DOC: Noisy comments
    /// FIXME: Quit dialog issue
    pub fn dialog_dive(
        our_infos: Res<OurInfos>,
        
        mut dialog_dive_event: EventReader<DialogDiveEvent>,

        mut panel_query: Query<&mut DialogPanel, With<Animator<Style>>>,
        upper_scroll_query: Query<&mut UpperScroll, With<Scroll>>,

        mut drop_first_text_upper_scroll_event: EventWriter<DropFirstTextUpperScroll>,
        // mut trigger_event: EventWriter<TriggerEvent>,
    ) {
        for DialogDiveEvent { child_index, skip } in dialog_dive_event.iter() {
            info!("DEBUG: DialogDive Event");
            let mut panel = panel_query.single_mut();
            // let interlocutor = panel.main_interlocutor;

            let dialog_panel = panel.dialog_tree.clone();

            if dialog_panel.is_empty() {
                warn!(
                    "Empty DialogTree; The Interlocutor is still in dialog but has nothing to say."
                );

                // force the chnage detection
                panel.dialog_tree.clear();
                warn!("DEBUG:force clear dialog panel");
            } else {
                let dialog_tree = init_tree_file(dialog_panel.to_owned(), our_infos.clone());
                let upper_scroll = upper_scroll_query.single();

                // option 1: if it is the very last text of the dialog
                // or
                // option 2: if the monologue is not finished (except the last text `> 1`)
                // then drop it

                // option 2: (precision)
                // if there is at least 2 elem in the upper scroll
                // XXX: after selecting a choice, this test will **normally** always be ignored
                // cause can't be in a choice phase while having text left in the UpperScroll
                // if not, the player could choose smth for 'nothing'

                if upper_scroll.texts.len() > 1 {
                    drop_first_text_upper_scroll_event.send(DropFirstTextUpperScroll);
                } else if !(dialog_tree.borrow().is_choice() && *skip) {
                    // shouldn't exist : end choice (which hasn't child)
                    // so, we don't test it here

                    // REFACTOR: Check if the Trigger event field is not empty before sending anything
                    // trigger_event.send(TriggerEvent(dialog_tree.borrow().trigger_event.clone()));

                    if dialog_tree.borrow().is_end_node() {
                        // will be handle by the update_dialog_panel system
                        // as Exit the Combat

                        panel.dialog_tree.clear();
                        info!("clear dialog panel");
                    } else {
                        // go down on the first child
                        // DOC: Specifics Rules link
                        // ignore the other child if there is one
                        // **the rule implied not**
                        // cause a text must have one child or none

                        let child = dialog_tree.borrow().children[*child_index]
                            .borrow()
                            .print_file();

                        panel.dialog_tree = child;
                    }
                }
            }
        }
    }

    /// Disables empty button,
    /// (hidden == disable)
    ///
    /// Prevents checking a index in the choices list.
    pub fn hide_empty_button(
        mut button_query: Query<(Entity, &mut Visibility, &PlayerChoice), With<Button>>,

        player_scroll_query: Query<
            (Entity, &PlayerScroll, &Children),
            // if the choices field is modified
            (Changed<PlayerScroll>, With<Scroll>),
        >,
    ) {
        for (_, player_scroll, children) in player_scroll_query.iter() {
            for button in children.iter() {
                match button_query.get_mut(*button) {
                    // FIXME: handle this error
                    Err(e) => warn!("Err: A Player Scroll's child is not a button: {:?}", e),
                    Ok((_, mut visibility, player_choice)) => {
                        // REFACTOR: just deref it
                        let choice_index = player_choice.0;
                        let choices = player_scroll.choices.clone();

                        *visibility = if choice_index < choices.len() {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        };

                        info!(
                            "button °{:?} visibility switch: {:?}",
                            choice_index,
                            *visibility == Visibility::Inherited
                        );
                    }
                }
            }
        }
    }

    // /// DOC
    // /// TODO: feat - Add triggerEvent (and add to the app)
    // ///
    // /// Options
    // ///
    // /// - Match the enum into handle it direclty
    // /// - Match the enum into throw the correct event
    // pub fn throw_trigger_event(mut trigger_event: EventReader<TriggerEvent>) {
    //     for TriggerEvent(triggers) in trigger_event.iter() {
    //         for event_to_exec in triggers.iter() {
    //             match event_to_exec {
    //                 ThrowableEvent::FightEvent => {
    //                     info!("Fight Event")
    //                 }
    //                 ThrowableEvent::HasFriend => {
    //                     info!("Has Friend Event")
    //                 }
    //             }
    //         }
    //     }
    // }

    /// Drops first text from the UpperScroll
    pub fn drop_first_text_upper_scroll(
        mut drop_event: EventReader<DropFirstTextUpperScroll>,

        mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,

        // update the DialogBox according to the Scroll
        mut scroll_event: EventWriter<UpdateScrollEvent>,
    ) {
        for DropFirstTextUpperScroll in drop_event.iter() {
            // TOTEST: calling this event mean the scroll do exist but maybe not ?
            let (mut upper_scroll, _upper_scroll_entity) = upper_scroll_query.single_mut();

            if let Some((_first, rem)) = upper_scroll.texts.split_first() {
                // pop first only
                upper_scroll.texts = rem.to_vec();

                // ask to update the content of scroll (which will update the DialogBox)
                scroll_event.send(UpdateScrollEvent);
            } else {
                // shouldn't be the case
                warn!("The UpperScroll does not contain text")
            }
        }
    }
}

mod dialog_scroll {
    //! Scrolls

    use bevy::prelude::*;

    use crate::{
        dialog_box::ResetDialogBoxEvent,
        dialog_panel::DialogPanelResources,
        constants::ui::dialogs::SCROLL_ANIMATION_FRAMES_NUMBER
    };

    /// Any scroll should have this component.
    ///
    /// Used to animate scroll.
    ///
    /// # Note
    ///
    /// Can't merge the PlayerScroll and UpperSrcoll int othe Scroll Component,
    /// due to quering manners, and update scrolls function being to different
    /// from one scroll to another.
    ///
    /// Cause a serie of text is just a monologue and we don't care
    /// about the previous text displayed.
    /// All choice need to be prompted (not especially on the same page).
    #[derive(Component)]
    pub struct Scroll {
        pub current_frame: usize,
        pub reverse: bool,
    }

    #[derive(Component, Deref, DerefMut)]
    pub struct ScrollTimer(pub Timer);

    /// Saves every text, in order, contained in the current dialog node.
    #[derive(Component, Reflect)]
    pub struct UpperScroll {
        pub texts: Vec<String>,
    }

    /// Saves all choice we could have to display
    #[derive(Component, Reflect)]
    pub struct PlayerScroll {
        pub choices: Vec<String>,
    }

    /// Represents all button which may contain choice for the player to made
    #[derive(Component)]
    pub struct PlayerChoice(pub usize);

    /// Happens when
    ///   - ui::dialog_panel::create_dialog_panel
    ///     - UI Wall creation
    ///   - ui::dialog_panel::update_dialog_panel
    ///     - After any change in the dialog tree
    ///     ( except when the d. tree is empty )
    ///   - ui::dialog_player::drop_first_text_upper_scroll
    ///     - ask to update the upper scroll after droping one text
    ///
    /// Read in
    ///   - ui::dialog_panel::update_upper_scroll
    ///     - create a dialogBox with the text contained in the UpperScroll,
    ///     or update Text in existing dialogBox.
    ///   - ui::dialog_panel::update_player_scroll
    ///     - update choices displayed in the player scroll.
    pub struct UpdateScrollEvent;

    /// # Note
    ///
    /// Waiting for the use of spritesheet in bevy ui.
    /// To stop using frame by frame update.
    pub fn animate_scroll(
        time: Res<Time>,
        // texture_atlases: Res<Assets<TextureAtlas>>,
        dialog_panel_resources: Res<DialogPanelResources>,
        mut commands: Commands,
        mut scroll_query: Query<
            (&mut UiImage, &mut Scroll, &mut ScrollTimer, Entity),
            (With<UpperScroll>, Without<PlayerScroll>),
        >,
    ) {
        for (mut image, mut scroll, mut timer, entity) in scroll_query.iter_mut() {
            timer.tick(time.delta());

            if timer.finished() {
                if scroll.reverse {
                    scroll.current_frame -= 1;

                    if scroll.current_frame == 0 {
                        commands.entity(entity).remove::<ScrollTimer>();
                    }
                } else {
                    scroll.current_frame += 1;

                    if scroll.current_frame >= SCROLL_ANIMATION_FRAMES_NUMBER - 1 {
                        commands.entity(entity).remove::<ScrollTimer>();
                    }
                }

                image.texture =
                    dialog_panel_resources.scroll_animation[scroll.current_frame].clone();
            }
        }
    }

    /// Displays the content of the first element contained in the Upper Scroll
    ///
    /// # Note
    ///
    /// TODO: feature - execute animation when any update occurs; (Handle by event allow it)
    /// For example, the closure opening to clear and display.
    pub fn update_upper_scroll(
        mut scroll_event: EventReader<UpdateScrollEvent>,

        // mut scroll_query: Query<(Or<&PlayerScroll, &mut UpperScroll>, Entity), With<Scroll>>,
        upper_scroll_query: Query<(&UpperScroll, Entity), With<Scroll>>,

        mut reset_event: EventWriter<ResetDialogBoxEvent>,
    ) {
        for UpdateScrollEvent in scroll_event.iter() {
            info!("- Upper - Scroll Event !");

            match upper_scroll_query.get_single() {
                Err(e) => warn!("{}", e),
                Ok((upper_scroll, upper_scroll_entity)) => {
                    // let text = upper_scroll.texts.pop();
                    // just collect the first without removing it
                    match upper_scroll.texts.first() {
                        None => {
                            info!("empty upper scroll");
                            // TODO: feature - send event to close (reverse open) upper scroll ?
                        }
                        Some(dialog_box_text) => {
                            info!("upper scroll gain a text");

                            reset_event.send(ResetDialogBoxEvent {
                                dialog_box: upper_scroll_entity,
                                text: dialog_box_text.to_owned(),
                            });
                        }
                    }
                }
            }
        }
    }

    /// Player scroll can contain multiple choice
    /// that will be displayed at the same time.
    ///
    /// For each choice, resets the DialogBox associated with its index
    pub fn update_player_scroll(
        mut scroll_event: EventReader<UpdateScrollEvent>,

        player_scroll_query: Query<(&PlayerScroll, &Children, Entity), With<Scroll>>,

        mut reset_event: EventWriter<ResetDialogBoxEvent>,
    ) {
        for UpdateScrollEvent in scroll_event.iter() {
            info!("- Player - Scroll Event !");

            match player_scroll_query.get_single() {
                Err(e) => warn!("{}", e),
                Ok((player_scroll, scroll_children, _player_scroll_entity)) => {
                    // REFACTOR: every 3 choices create a page and start again from the 1st child
                    for (place, choice) in player_scroll.choices.iter().enumerate() {
                        // FIXME: CRASH HERE OutOfBound if place > 3 (view the refactor above)
                        if place > 3 {
                            break;
                        }

                        // The button's visibility is based on the size
                        // of the vector: player_scroll.choices
                        reset_event.send(ResetDialogBoxEvent {
                            dialog_box: scroll_children[place],
                            text: choice.to_owned(),
                        });
                    }
                    // info!("DEBUG: player scroll gain {} choice-s", place);
                }
            }
            // if no choice
            // info!("empty player scroll");
            // send event to close (reverse open) upper scroll ?
        }
    }
}

mod constants {
    pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

    pub const FIXED_TIME_STEP: f32 = 1. / 60.;

    pub const HEIGHT: f32 = 720.;
    pub const RESOLUTION: f32 = 16. / 9.;

    pub mod character {
        pub const KARMA_MAX: i32 = 100;
        pub const KARMA_MIN: i32 = -KARMA_MAX;

        pub mod dialog {
            // Flibittygibbit

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

            }
    }

    pub mod ui {
        pub mod dialogs {
            use bevy::prelude::Color;

            pub const DIALOG_PANEL_ANIMATION_OFFSET: f32 = -1000.;
            pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
            pub const DIALOG_PANEL_ANIMATION_TIME_MS: u64 = 500;
            pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
            pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

            pub const NORMAL_BUTTON: Color = Color::rgba(0.01, 0.01, 0.01, 0.01);
            pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
            pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
        }
    }
}
