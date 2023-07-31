//! Dialog System
//!
//! Complex
//!
//! - A struct DialogNode can be insert to an entity
//!   - This Node may contains
//!     - some Text
//!     - some Choice
//!   - A specific Dialog can have some conditon
//!     - Karma based
//!     - Event based
//!     - Choice based
//!   - A node can send Specific Event
//!
//! Tree structure based on [`Rc<RefCell<Node>>`](https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75)
#![warn(missing_docs)]

use std::{cell::RefCell, fmt, rc::Rc, str::FromStr};

use bevy::prelude::{warn, Component, Deref, DerefMut};

const KARMA_MAX: i32 = 100;
const KARMA_MIN: i32 = -KARMA_MAX;

/// Contains the current Dialog node of the interlocutor's talk.
///
/// Holds a String which can be converted to a `Rc<RefCell<DialogNode>>`
/// by `print_file()`
///
/// # Example
///
/// IDEA: In the example, put the struct into a `commands.spawn((...))` method
///
/// ```rust
/// use fto_dialog::Dialog;
/// let dialog = Dialog::from_str("# Fabien\n\n- Hello\n");
/// ```
///
/// # Note
///
/// DOC: Struct might be useless
/// NOTE: CUSTOM
///
/// To extend this `Component`, don't import it.... and create your own.
///
/// ```rust
/// use bevy::prelude::Component;
///
/// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default, Component)]
/// struct YourDialog {
///     root: Option<String>,
///     current_node: Option<String>,
/// }
/// ```
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

    // pub fn new_string(string: String) -> Dialog {
    //     Dialog {
    //         current_node: Some(string),
    //     }
    // }

    /// Returns the read-only current node (an `&Option<String>`)
    pub fn current_node(&self) -> &Option<String> {
        &self.current_node
    }

    /// Returns the mutable reference current node (an `&Option<String>`)
    pub fn current_node_mut(&mut self) -> &mut Option<String> {
        &mut self.current_node
    }
}

/// An item from a `DialogNode`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DialogContent {
    /// A line from a `DialogNode`
    Text(String),
    /// A choice from a `DialogNode`
    Choice {
        /// Choice's text
        text: String,
        /// A potential condition which could lock the choice access
        condition: Option<DialogCondition>,
    },
}

impl Default for DialogContent {
    fn default() -> Self {
        DialogContent::Text(String::new())
    }
}

impl DialogContent {
    /// Construct a new empty Text.
    pub fn new_text() -> DialogContent {
        DialogContent::default()
    }

    /// Construct a new empty Choice.
    pub fn new_choice() -> DialogContent {
        DialogContent::Choice {
            text: String::new(),
            condition: None,
        }
    }

    /// Only compare the type,
    /// it don't compare content
    pub fn eq(&self, comp: DialogContent) -> bool {
        matches!(
            (self.clone(), comp),
            (DialogContent::Text(_), DialogContent::Text(_))
                | (DialogContent::Choice { .. }, DialogContent::Choice { .. })
        )
    }

    /// Returns `true` if this content is a choice.
    pub fn is_choice(&self) -> bool {
        matches!(
            self.clone(),
            DialogContent::Choice {
                text: _text,
                condition: _cond,
            }
        )
    }

    /// Returns `true` if this content is a text.
    pub fn is_text(&self) -> bool {
        matches!(self.clone(), DialogContent::Text(_))
    }
}

/// World event
///
/// NOTE: CUSTOM
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GameEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
    HasCharisma,
    HasFriend,
}

impl fmt::Display for GameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameEvent::BeatTheGame => write!(f, "BeatTheGame"),
            GameEvent::FirstKill => write!(f, "FirstKill"),
            GameEvent::AreaCleared => write!(f, "AreaCleared"),
            GameEvent::HasCharisma => write!(f, "HasCharisma"),
            GameEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for GameEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<GameEvent, Self::Err> {
        match input {
            "BeatTheGame" => Ok(GameEvent::BeatTheGame),
            "FirstKill" => Ok(GameEvent::FirstKill),
            "AreaCleared" => Ok(GameEvent::AreaCleared),
            "HasCharisma" => Ok(GameEvent::HasCharisma),
            "HasFriend" => Ok(GameEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// Happens in
///   - ui::dialog_player
///     - dialog_dive
///     Exit a node
///
/// Read in
///   - ui::dialog_player
///     - ~~throw_trigger_event~~
///     Match the Enum and handle it
///     REFACTOR: or/and TriggerEvent Handle by sending these real Event
pub struct TriggerEvent(pub Vec<ThrowableEvent>);
// pub struct FightEvent;

/// List all triggerable event,
/// that can be send when quitting a dialog node
///
/// # Note
///
/// NOTE: CUSTOM
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ThrowableEvent {
    FightEvent,
    HasFriend,
}

impl fmt::Display for ThrowableEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThrowableEvent::FightEvent => write!(f, "FightEvent"),
            ThrowableEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for ThrowableEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<ThrowableEvent, Self::Err> {
        match input {
            "FightEvent" => Ok(ThrowableEvent::FightEvent),
            "HasFriend" => Ok(ThrowableEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// Choice's Condition
///
/// # Note
///
/// NOTE: CUSTOM
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DialogCondition {
    /// `(0,0) = infinite / no threshold`
    ///
    /// A dialog choice with a condition karma_threshold at (0,0)
    /// will always be prompted
    karma_threshold: Option<(i32, i32)>,
    event: Option<Vec<GameEvent>>,
}

impl DialogCondition {
    /// Construct a new DialogCondition with the given `karma_threshold` and `event`.
    pub fn new(
        karma_threshold: Option<(i32, i32)>,
        event: Option<Vec<GameEvent>>,
    ) -> DialogCondition {
        DialogCondition {
            karma_threshold,
            event,
        }
    }

    /// Only check if the `karma` is within the range of the condition
    pub fn is_verified(&self, karma: i32) -> bool {
        // TODO: feature - also check if its event has been already triggered in the game

        match self.karma_threshold {
            None => true,
            Some(karma_threshold) => karma >= karma_threshold.0 && karma <= karma_threshold.1,
        }
    }

    /// Returns the read-only `karma_threshold` item of the `DialogCondition`.
    pub fn karma_threshold(&self) -> &Option<(i32, i32)> {
        &self.karma_threshold
    }

    /// Returns the mutable reference `karma_threshold` item of the `DialogCondition`.
    pub fn karma_threshold_mut(&mut self) -> &mut Option<(i32, i32)> {
        &mut self.karma_threshold
    }

    /// Returns the read-only `event` item of the `DialogCondition`.
    pub fn event(&self) -> &Option<Vec<GameEvent>> {
        &self.event
    }

    /// Returns the mtable reference `event` item of the `DialogCondition`.
    pub fn event_mut(&mut self) -> &mut Option<Vec<GameEvent>> {
        &mut self.event
    }
}

/// A tree node with
///
/// - `dialog_content`: a vector of either Text only or Choice only.
/// - `author`: the potential name of the speaker.
/// - `children`: its roots.
/// - `parent`: its origin.
/// - `trigger_event`: All events triggered by this dialogue.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct DialogNode {
    /// A vector of either Text only or Choice only.
    ///
    /// - A Roaster of answer enable
    ///   - each with certain conditions to be enabled
    /// - A Roaster of line from a monologue.
    ///   - TODO: with a priority system to alloy important event
    pub dialog_content: Vec<DialogContent>,
    /// The potential name of the speaker.
    pub author: Option<String>,
    /// Vector of its subtree.
    pub children: Vec<Rc<RefCell<DialogNode>>>,
    /// Its potential Parent.
    ///
    /// # Note
    ///
    /// maybe too much (prefer a stack in the TreeIterator)
    pub parent: Option<Rc<RefCell<DialogNode>>>,
    /// All events triggered by this dialogue.
    ///
    /// # Note
    ///
    /// NOTE: CUSTOM (but how...)
    pub trigger_event: Vec<ThrowableEvent>,
}

impl DialogNode {
    // pub fn is_empty(&self) -> bool {
    //     self.dialog_content.is_empty()
    // }

    /// # Return
    ///
    /// `true` if this node doesn't have any children.
    pub fn is_end_node(&self) -> bool {
        self.children.is_empty()
    }

    /// # Return
    ///
    /// `true` if the type of the first element (of dialog_content) is choice
    pub fn is_choice(&self) -> bool {
        if !self.dialog_content.is_empty() {
            return self.dialog_content[0].is_choice();
        }
        false
    }

    /// # Return
    ///
    /// `true` if the type of the first element (of dialog_content) is choice
    pub fn is_text(&self) -> bool {
        if !self.dialog_content.is_empty() {
            return self.dialog_content[0].is_text();
        }
        false
    }

    /// Put `new_node` in the end of the children vector.
    pub fn add_child(&mut self, new_node: Rc<RefCell<DialogNode>>) {
        self.children.push(new_node);
    }

    /// Give the author of the node.
    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }

    /// # Convention
    ///
    /// ```markdown
    /// # Author 1
    ///
    /// - first element of a text
    /// - second element of a text
    ///
    /// -> Event
    ///
    /// ### Author 2
    ///
    /// - first choice for the author 2 only enable when his/her karma is low | karma: -50,0;
    /// - second choice ... only enable when the event OlfIsGone has already occurs | event: OlfIsGone;
    /// - thrid choice always enable | None
    /// - fourth choice with a high karma and when the events OlfIsGone and PatTheDog already occurs | karma: 10,50; event: OlfIsGone, PatTheDog;
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the first choice is selected by the player
    /// - This DialogNode (and the three other below) is a direct child of the second DialogNode
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the second choice is selected by the player
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the third choice is selected by the player
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the fourth choice is selected by the player
    ///
    /// ```
    pub fn print_file(&self) -> String {
        self.print_file_aux(String::from("#"))
    }

    fn print_file_aux(&self, headers: String) -> String {
        let mut res = headers.clone();

        let character: String = match &self.author {
            Some(name) => " ".to_string() + name,

            None => String::from(" Narator"),
        };
        res.push_str(&character);
        res.push_str("\n\n");

        for dialog in &self.dialog_content {
            if let DialogContent::Text(text) = dialog {
                res.push_str("- ");
                res.push_str(text);
            } else if let DialogContent::Choice { text, condition } = dialog {
                res.push_str("- ");
                res.push_str(text);
                res.push_str(" | ");

                match condition {
                    Some(dialog_condition) => {
                        if let Some((min, max)) = dialog_condition.karma_threshold {
                            res.push_str(&format!(
                                "karma: {},{}; ",
                                &min.to_string(),
                                &max.to_string()
                            ));
                        }

                        match &dialog_condition.event {
                            Some(events) => {
                                if !events.is_empty() {
                                    // DOC: events in plurial ?
                                    res.push_str("event: ");
                                    for event in events {
                                        res.push_str(&event.to_string());
                                        res.push(',');
                                    }
                                    res.push(';');
                                    res = res.replace(",;", ";");
                                }
                            }
                            None => {}
                        }
                    }

                    None => {
                        res.push_str("None");
                    }
                }
            }
            res.push('\n');
        }
        res.push_str("\n\nEND");

        // event

        let children = &self
            .children
            .iter()
            .map(|tn| tn.borrow().print_file_aux(headers.clone() + "#"))
            .collect::<Vec<String>>()
            .join("\n\n");

        res.push_str(children);

        // smooth the end of phase
        res = res.replace(" \n", "\n");

        res = res.replace("#\n\n#", "##");
        res = res.replace("\n\n\n", "\n\n");

        // remove the last "\n\n"
        res = res.replace("END#", "#");
        // keep only one last '\n' (to respect init rules)
        res = res.replace("\n\nEND", "\n");

        res
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
enum InitPhase {
    #[default]
    AuthorPhase,
    ContentPhase,
    ConditionPhase,
}

/// # Argument
///
/// * `s` - A string that holds a DialogTree
///
/// IDEA: To allow users to choose The max/min of Karma put it in Argument
///
/// # Return
///
/// the root: `Rc<RefCell<DialogNode>>`
///
/// REFACTOR: `Result<Rc<RefCell<DialogNode>>, Self::Err>`
///
/// # Panics
///
/// The creation will panic
/// if any argument to the process is not valid [DialogTree format].
///
/// REFACTOR: Send an error, don't crash
///
/// # Conventions
///
/// ## Dialog Creation
///
/// A Dialog can involve as many characters as you like.
/// A Dialog's Node is composed of:
///
/// - the author
/// - the content
///   - A serie of text;
///   To create a monologue.
///   - A serie of choice.
///
///     If the author of this serie is
///     - a NPC:
///     The choice will be selected by a priority system,
///     Important Event dialog before random dialog.
///     - the Main Character:
///     The choices to be made will be indicated to the player.
///     In The player scroll at the bottom the UI Wall.
/// - the event the Node will launch when it's exited.
///
/// ## Rules
///
/// - A choice is made of
///   - a text
///   - condition; Must end by `;` if != None
///     - karma threshold; (x,y) with x, y ∈ N.
///     The player's karma must be within this certain range
///     - event;
///     All followed events must be triggered to enable this choice.
/// - A text can have only one child
/// - A dialog node cannot have more than one type of dialog_content
///   - for example
///   within a same DialogNode, having a text and a choice in the dialog_content field
///   will break soon or later
///   FIXME: make it break soon as possible - Create Error
///
/// ***The end marker of the dialog is `\n`.***
/// You **MUST** end this string by a `\n`.
///
/// ## Tips
///
/// - You can type
///   - `k: x,y;` instead of `karma: x,y;`
///   - `e: Event1, Event2;` instead of `event: Event1, Event2;`
/// - You can use `MAX`/`MIN` to pick the highest/lowest karma threshold possible
///   - REFACTOR: use macro by example or put these in the method argument
/// - Prefere not typing anything if it's something like this: `k: MIN,MAX;`
/// - No matter in the order: `karma: 50,-50;` will result by `karma_threshold: Some((-50,50))`
/// - To include these symbols `-`, `\n`, `|`, `#` or `>` put the char `/` just before
///
/// # Examples
///
/// The MainCharacter's catchphrase is followed
/// by two possible outcomes/choices for the interlocutor.
///
/// - a generic one
///   - random chill dialog,
///   a simple Text("I have to tell something")
/// - a huge text to cheer the fact that Olf's reign is over
///   - only enable when the event `Olf's takedown` occurs
///   - a first simple Text with a second simple Text as child
///
/// ```rust
/// # main() -> Result<(), Self::Err> {
/// let tree: Rc<RefCell<DialogNode>> = init_tree_file(
///     String::from(
/// "# Morgan
///
/// - Hello
///
/// ### Fabien lae Random
///
/// - I have to tell something | None
/// - You beat Olf ! | event: OlfBeaten;
///
/// #### Fabien lae Random
///
/// - Something
///
/// #### Fabien lae Random
///
/// - Now you can chill at the hospis
///
/// -> OpenTheHospis\n"
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// To create a long monologue,
/// avoid using `\n`, prefere using `-`
/// to seperated paragraph
///
/// ```rust
/// # use fto_dialog::DialogNode;
/// let tree: Rc<RefCell<DialogNode>> = init_tree_file(
///     String::from(
/// "# Olf
///
/// - Hello
/// - Do you mind giving me your belongings ?
/// - Or maybe...
/// - You want to fight me ?
///
/// ### Morgan
///
/// - Here my money | e: WonTheLottery;
/// - You will feel my guitar | None
/// - Call Homie | k: 10,MAX;
///
/// #### Olf
///
/// - Thank you very much
///
/// #### Olf
///
/// - Nice
/// -> FightEvent
///
/// #### Olf
///
/// - Not Nice
/// -> FightEvent\n"
///     )
/// );
/// ```
///
/// [DialogTree format]: #rules
pub fn init_tree_file(s: String) -> Rc<RefCell<DialogNode>> {
    // Result<Rc<RefCell<DialogNode>>, Box<dyn Error>> {
    // Result<Rc<RefCell<DialogNode>>, &'static str> {
    let root = Rc::new(RefCell::new(DialogNode::default()));

    let mut current = root.clone();

    let mut author = String::new();
    let mut save = String::new();
    // k/karma or e/event
    let mut condition = DialogCondition::default();
    // CANT due to reading text one letter/number by one: avoid using karma as a string into .parse::<i32>().unwrap()
    // let mut negative_karma = false;
    let mut karma = String::new();
    let mut event = String::new();

    // init with text
    let mut dialog_content: DialogContent = DialogContent::new_text();

    // Check if a given char should be insert as text
    // allow to have text with special char (like - / # )
    let mut except = false;

    let mut header_numbers = 0;
    let mut last_header_numbers = 0;

    let mut phase: Option<InitPhase> = None;

    let mut karma_phase = false;
    let mut event_phase = false;
    // allow to write `e:` or `ejhlksdfh:` instead of `event:`
    let mut post_colon = false;

    let mut trigger_phase = false;

    // let mut new_line = false;

    let chars = s.chars().collect::<Vec<char>>();
    // println!("{}", chars.len());
    // println!("{}", s);

    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < chars.len())
    {
        // println!("c: {}", *c);

        /* -------------------------------------------------------------------------- */
        /*                                 Transitions                                */
        /* -------------------------------------------------------------------------- */

        if *c == '#' && !except {
            header_numbers += 1;
            phase = Some(InitPhase::AuthorPhase);
        }
        // phase != InitPhase::ConditionPhase to permit negative number in the karma threshold
        else if *c == '-' && phase != Some(InitPhase::ConditionPhase) && !except {
            phase = Some(InitPhase::ContentPhase);
        } else if *c == '>' && phase == Some(InitPhase::ContentPhase) && !except {
            phase = None;
            trigger_phase = true;
        } else if *c == '|' && phase == Some(InitPhase::ContentPhase) && !except {
            while save.starts_with(' ') {
                save.remove(0);
            }

            dialog_content = DialogContent::new_choice();

            phase = Some(InitPhase::ConditionPhase);
        } else if *c == ',' && karma_phase && phase == Some(InitPhase::ConditionPhase) {
            // can be solved with karma_min, karma_max
            // println!("karma 1rst elem: {}", karma);
            let k: i32;
            // karma = karma.to_uppercase();
            k = if karma == *"MAX" || karma == *"max" {
                KARMA_MAX
            } else if karma == *"MIN" || karma == *"min" {
                KARMA_MIN
            } else {
                karma.parse::<i32>().unwrap()
            };
            condition.karma_threshold = Some((k, KARMA_MAX));

            // reuse this variable
            karma.clear();
        }
        /* -------------------------------------------------------------------------- */
        /*                                End of Phase                                */
        /* -------------------------------------------------------------------------- */
        else if *c == '\n' && phase == Some(InitPhase::AuthorPhase) {
            while author.starts_with(' ') {
                author.remove(0);
            }
            // println!("author: {}", author);

            // root: header_numbers != 1
            if last_header_numbers != 0 {
                // println!("previous:\n{}", current.borrow().print_file());
                // println!("last_header_numbers {}", last_header_numbers);
                // println!("header_numbers {}", header_numbers);

                /* set current to the parent of the incomming node
                 * if last_header_numbers - header_numbers +1 == 0;
                 *     cause 0..0 = 0 iteration
                 * then current = incomming node's parent
                 * so skip this step
                 */
                let limit = last_header_numbers - header_numbers + 1;
                for _step in 0..limit {
                    // println!("step {}", _step);
                    // should not panic anyway
                    let parent = current.clone().borrow().parent.clone();
                    // println!(
                    //     "parent {}:\n{}",
                    //     _step,
                    //     parent.clone().unwrap().borrow().print_file()
                    // );
                    current = parent.clone().unwrap();

                    // match &current.borrow_mut().parent {
                    //     Some(parent) => current.borrow_mut() = parent.to_owned(),
                    //     None => panic!("no parent"),
                    // }
                }

                let child = Rc::new(RefCell::new(DialogNode::default()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }

                // setting up the *second* link parent
                child.borrow_mut().parent = Some(current);

                // go down into a new child
                current = child;
            }

            // REFACTOR: give the real entity_id or remove id
            current.borrow_mut().author = Some(author.to_owned());

            author.clear();

            last_header_numbers = header_numbers;
            header_numbers = 0;

            except = false;
            phase = None;
        } else if *c == ';' && karma_phase {
            // println!("karma: {}", karma);
            let k: i32 = if karma == *"MAX" || karma == *"max" {
                KARMA_MAX
            } else if karma == *"MIN" || karma == *"min" {
                KARMA_MIN
            } else {
                karma.parse::<i32>().unwrap()
            };

            match condition.karma_threshold {
                None => {
                    warn!("init creation condition has been skipped");
                }
                // _ cause the second `i32` is `KARMA_MAX` (placeholder)
                Some((x, _)) => {
                    if x <= k {
                        condition.karma_threshold = Some((x, k));
                    } else {
                        condition.karma_threshold = Some((k, x));
                    }
                }
            }
            karma.clear();

            karma_phase = false;
        } else if (*c == ';' || *c == ',') && event_phase {
            // println!("event : {}", event);
            let e = event.parse::<GameEvent>().unwrap();
            match condition.event {
                Some(vec) => {
                    let mut events = vec.clone();
                    events.push(e);
                    condition.event = Some(events);
                }
                None => condition.event = Some(vec![e]),
            }

            // println!("event cleared");
            event.clear();

            if *c == ';' {
                event_phase = false;
            }
        } else if *c == '\n'
            && phase == Some(InitPhase::ConditionPhase)
            && dialog_content.is_choice()
        // && !karma_phase
        // && !event_phase
        {
            // remove the space on the first position
            while save.starts_with(' ') {
                save.remove(0);
            }

            // remove the space on the last position
            while save.ends_with(' ') {
                save.remove(save.len() - 1);
            }

            // println!("choice inserted: {}", save);

            let choice: DialogContent = match (condition.karma_threshold, condition.clone().event) {
                (None, None) => DialogContent::Choice {
                    text: save.clone(),
                    condition: None,
                },
                (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => DialogContent::Choice {
                    text: save.clone(),
                    condition: Some(condition.clone()),
                },
            };

            current.borrow_mut().dialog_content.push(choice);

            phase = None;

            save.clear();
            condition = DialogCondition::default();
        } else if *c == '\n'
            && phase == Some(InitPhase::ContentPhase)
            && !save.is_empty()
            && dialog_content.is_text()
            && !except
        {
            // remove the space on the first position
            while save.starts_with(' ') {
                save.remove(0);
            }

            // remove the space on the last position
            while save.ends_with(' ') {
                save.remove(save.len() - 1);
            }

            // println!("text inserted: {}", save);

            let dialog = DialogContent::Text(save.clone());
            current.borrow_mut().dialog_content.push(dialog);

            save.clear();

            phase = None;
        } else if (*c == ',' || *c == '\n') && trigger_phase {
            // println!("event : {}", event);
            let e = event.parse::<ThrowableEvent>().unwrap();

            // add the triggered event to the vector of the current DialogNode
            current.borrow_mut().trigger_event.push(e);

            // println!("event cleared");
            event.clear();

            if *c == '\n' {
                trigger_phase = false;
            }
        }
        // read text or condition
        else if c.is_ascii() || c.is_alphanumeric() {
            // the except char is /
            if *c == '/' {
                // info!("except");
                except = true;
            }
            // `;` or `\n` put an end to the selection of condtion
            else if phase == Some(InitPhase::ConditionPhase) {
                if *c == 'k' {
                    karma_phase = true;
                    post_colon = true;
                } else if *c == 'e' && !event_phase {
                    event_phase = true;
                    post_colon = true;
                } else if *c == ':' && (event_phase || karma_phase) {
                    post_colon = false;
                }
                // c.is_numeric() ||
                // authorize MAX or MIN input
                else if karma_phase && !post_colon && *c != ' '
                // || *c == '-'
                {
                    // negative symbol -
                    karma.push(*c);

                    println!("k: {}", karma);
                } else if event_phase && *c != ' ' && !post_colon {
                    event.push(*c);

                    // println!("e: {}", event);
                }
            } else if trigger_phase && *c != ' ' {
                event.push(*c);
            }
            // ignore the new line: "\n"
            // the \n is a marker to end some phase
            else if *c == '\n' && !except {
                // new_line = true;
                // println!("skip");

                // full reset

                dialog_content = DialogContent::new_text();

                phase = None;
            } else if phase == Some(InitPhase::AuthorPhase) && *c != '#' {
                author.push(*c);
            }
            // if !is_special_char_file(*c) || (is_special_char_file(*c) && except)
            else {
                save.push(*c);
                except = false;

                // println!("add {} ", *c);
            }
        }
    }

    root
}
