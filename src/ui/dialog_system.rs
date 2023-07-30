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
//! Tree structure based on https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75

use std::{cell::RefCell, fmt, rc::Rc, str::FromStr};

use bevy::prelude::{info, warn, Component, Deref, DerefMut};

const KARMA_MAX: i32 = 100;
const KARMA_MIN: i32 = -KARMA_MAX;

/// Contains the current Dialog node of the interlocutor's talk.
///
/// Holds a String which can be converted to a Rc<RefCell<DialogNode>>
/// by `print_file()`
///
/// # Example
///
/// IDEA: In the example, put the struct into a `commands.spawn((...))` method
///
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
/// # use fto_dialog::ui::dialog_system::Dialog;
/// Dialog {
///     current_node: Some("# Fabien\n\n- Hello\n".to_string())
/// }
/// #     Ok(())
/// # }
/// ```
///
/// # Note
///
/// DOC: Struct might be useless
///
/// To extend this `Component`, don't import it.... and create your own.
///
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
/// \#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default, Component)]
/// struct YourDialog {
///     root: Option<String>,
///     current_node: Option<String>,
/// }
/// #     Ok(())
/// # }
/// ```
#[derive(Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default, Component)]
pub struct Dialog {
    pub current_node: Option<String>,
}

impl Dialog {
    /// TODO: feature - Read at dialog_file
    pub fn new(str: &str) -> Dialog {
        Dialog {
            current_node: Some(str.to_string()),
        }
    }

    // fn new_string(string: String) -> Dialog {
    //     Dialog {
    //         current_node: Some(string),
    //     }
    // }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DialogType {
    Text(String),
    Choice {
        text: String,
        condition: Option<DialogCondition>,
    },
}

impl DialogType {
    fn new_text() -> DialogType {
        DialogType::Text(String::new())
    }

    fn new_choice() -> DialogType {
        DialogType::Choice {
            text: String::new(),
            condition: None,
        }
    }

    /// Only compare the type,
    /// it don't compare content
    fn _eq(&self, comp: DialogType) -> bool {
        matches!(
            (self.clone(), comp),
            (DialogType::Text(_), DialogType::Text(_))
                | (DialogType::Choice { .. }, DialogType::Choice { .. })
        )
    }

    fn is_choice(&self) -> bool {
        matches!(
            self.clone(),
            DialogType::Choice {
                text: _text,
                condition: _cond,
            }
        )
    }

    fn is_text(&self) -> bool {
        matches!(self.clone(), DialogType::Text(_))
    }
}

// TODO: MOVE IT UP
#[derive(PartialEq, Clone, Copy, Debug)]
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
/// Read in
///   - ui::dialog_player
///     - throw_trigger_event
///     Match the Enum and handle it
///     REFACTOR: or/and TriggerEvent Handle by sending these real Event
pub struct TriggerEvent(pub Vec<ThrowableEvent>);
// pub struct FightEvent;

/// DOC
///
/// List all triggerable event,
/// that can be send when quitting a dialog node
#[derive(PartialEq, Clone, Copy, Debug)]
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

/// NOTE: Could be a CUSTOM
#[derive(PartialEq, Clone, Default, Debug)]
pub struct DialogCondition {
    /// `(0,0) = infinite / no threshold`
    ///
    /// A dialog choice with a condition karma_threshold at (0,0)
    /// will always be prompted
    karma_threshold: Option<(i32, i32)>,
    event: Option<Vec<GameEvent>>,
}

impl DialogCondition {
    pub fn new(
        karma_threshold: Option<(i32, i32)>,
        event: Option<Vec<GameEvent>>,
    ) -> DialogCondition {
        DialogCondition {
            karma_threshold,
            event,
        }
    }

    pub fn is_verified(&self, karma: i32) -> bool {
        // TODO: feature - also check if its event has been already triggered in the game

        match self.karma_threshold {
            None => true,
            Some(karma_threshold) => karma >= karma_threshold.0 && karma <= karma_threshold.1,
        }
    }

    pub fn karma_threshold(&self) -> &Option<(i32, i32)> {
        &self.karma_threshold
    }

    pub fn karma_threshold_mut(&mut self) -> &mut Option<(i32, i32)> {
        &mut self.karma_threshold
    }

    pub fn event(&self) -> &Option<Vec<GameEvent>> {
        &self.event
    }

    pub fn event_mut(&mut self) -> &mut Option<Vec<GameEvent>> {
        &mut self.event
    }
}

/// Points to the first DialogNode
/// Used to be linked with an entity
#[derive(PartialEq, Clone, Debug)]
pub struct DialogTree {
    pub current: Option<DialogNode>,
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct DialogNode {
    /// Choice can have multiple children (to give a real impact to the choice)
    ///
    /// Every character can have multitude choice
    ///
    /// - A Roaster of answer enable for the player
    ///   - each with certain conditions to be enabled
    /// - A Roaster of catchphrase for npc
    ///   - with a priority system to alloy important event
    ///   to take over the ambiance
    ///   - it will permit more life and warm content,
    ///   some not cold npc to grow with the place
    ///
    /// # Note
    ///
    /// DOC: Rename DialogNode's field: dialog_type
    pub dialog_type: Vec<DialogType>,
    /// Actor / Actress
    ///
    /// can be an npc OR a player.
    ///
    /// The u32 is the id of the entity,
    /// The String is their name
    ///
    /// # Note
    ///
    /// REFACTOR: Remove the execution-time known variable id
    pub author: Option<(u32, String)>,
    pub children: Vec<Rc<RefCell<DialogNode>>>,
    /// maybe too much (prefer a stack in the TreeIterator)
    pub parent: Option<Rc<RefCell<DialogNode>>>,
    pub trigger_event: Vec<ThrowableEvent>,
}

impl DialogNode {
    // pub fn is_empty(&self) -> bool {
    //     self.dialog_type.is_empty()
    // }

    pub fn is_end_node(&self) -> bool {
        self.children.is_empty()
    }

    /// # Return
    ///
    /// `true` if the type of the first element (of dialog_type) is choice
    pub fn is_choice(&self) -> bool {
        if !self.dialog_type.is_empty() {
            return self.dialog_type[0].is_choice();
        }
        false
    }

    /// # Return
    ///
    /// `true` if the type of the first element (of dialog_type) is choice
    pub fn is_text(&self) -> bool {
        if !self.dialog_type.is_empty() {
            return self.dialog_type[0].is_text();
        }
        false
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<DialogNode>>) {
        self.children.push(new_node);
    }

    pub fn author(&self) -> Option<(u32, String)> {
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
            Some((_id, name)) => " ".to_string() + name,

            None => String::from(" Narator"),
        };
        res.push_str(&character);
        res.push_str("\n\n");

        for dialog in &self.dialog_type {
            if let DialogType::Text(text) = dialog {
                res.push_str("- ");
                res.push_str(text);
            } else if let DialogType::Choice { text, condition } = dialog {
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

enum InitPhase {
    AuthorPhase,
    ContentPhase,
    ConditionPhase,
}

/// # Argument
///
/// * `s` - A string that holds a DialogTree
///
/// # Panics
///
/// The creation will panic
/// if any argument to the process is not valid DialogTree format.
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
/// - A dialog node cannot have more than one type of dialog_type
///   - for example
///   within a same DialogNode, having a text and a choice in the dialog_type field
///   will break soon or later
///   FIXME: make it break soon as possible
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
/// - Prefere not typing anything if it's something like this: `k: MIN,MAX;`
/// - No matter in the order: `karma: 50,-50;` will result by `karma_threshold: Some((-50,50))`
/// - To use '-', '\n', '|', '#' or '>' prefere use the char '/' just before these chars
/// to prevent unwanted phase transition (from content phase to whatever phase)
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
/// # // ignore the extra '#' on the example (code block )
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree: DialogTree = init_tree_file(
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
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree: DialogTree = init_tree_file(
///     String::from(
/// "# Olf
///
/// - Hello
/// - Do you mind giving me your belongings ?
/// - Or maybe...
/// - You want to fight me ?
///
/// \## Morgan
///
/// - Here my money | e: WonTheLottery;
/// - You will feel my guitar | None
/// - Call Homie | k: 10,MAX;
///
/// \### Olf
///
/// - Thank you very much
///
/// \### Olf
///
/// - Nice
/// -> FightEvent
///
/// \### Olf
///
/// - Not Nice
/// -> FightEvent\n"
///     )
/// );
/// #     Ok(())
/// # }
/// ```
pub fn init_tree_file(s: String) -> Rc<RefCell<DialogNode>> {
    let root = Rc::new(RefCell::new(DialogNode::default()));

    let mut current = root.clone();

    let mut author = String::new();
    let mut save = String::new();
    // k, e, karma or event
    let mut condition = DialogCondition::default();
    // CANT due to reading text one letter/number by one: avoid using karma as a string into .parse::<i32>().unwrap()
    // let mut negative_karma = false;
    let mut karma = String::new();
    let mut event = String::new();

    // init with text
    let mut dialog_type: DialogType = DialogType::new_text();

    // Check if a given char should be insert as text
    // allow to have text with special char (like - / # )
    let mut except = false;

    let mut header_numbers = 0;
    let mut last_header_numbers = 0;

    // REFACTOR: Use InitPhase
    let mut author_phase = false;
    let mut content_phase = false;
    let mut condition_phase = false;

    let mut karma_phase = false;
    let mut event_phase = false;
    // allow to write `event:` or `ejhlksdfh:` instend of `e:`
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

        if (condition_phase || author_phase) && content_phase || (author_phase && condition_phase) {
            panic!("Illegal Combinaison of phase; author phase: {}, content phase: {}, condition phase: {}",
            author_phase, content_phase, condition_phase);
        }

        /* -------------------------------------------------------------------------- */
        /*                                 Transitions                                */
        /* -------------------------------------------------------------------------- */

        if *c == '#' && !except {
            header_numbers += 1;
            author_phase = true;
        }
        // !condition_phase to permit negative number in the karma threshold
        else if *c == '-' && !condition_phase && !except {
            content_phase = true;
        } else if *c == '>' && content_phase && !except {
            content_phase = false;
            trigger_phase = true;
        } else if *c == '|' && content_phase && !except {
            while save.starts_with(' ') {
                save.remove(0);
            }

            dialog_type = DialogType::new_choice();

            condition_phase = true;
            content_phase = !content_phase;
        } else if *c == ',' && karma_phase && condition_phase {
            // can be solved with karma_min, karma_max
            // println!("karma 1rst elem: {}", karma);
            let k: i32;
            // karma = karma.to_uppercase();
            if karma == *"MAX" || karma == *"max" {
                k = KARMA_MAX;
            } else if karma == *"MIN" || karma == *"min" {
                k = KARMA_MIN;
            } else {
                k = karma.parse::<i32>().unwrap();
            }
            condition.karma_threshold = Some((k, KARMA_MAX));

            // reuse this variable
            karma.clear();
        }
        //
        // End of Phase
        //
        else if *c == '\n' && author_phase {
            while author.starts_with(' ') {
                author.remove(0);
            }
            // println!("author: {}", author);

            // root: header_numbers != 1
            if last_header_numbers != 0 {
                // println!("previous:\n{}", current.borrow().print_file());
                // println!("last_header_numbers {}", last_header_numbers);
                // println!("header_numbers {}", header_numbers);

                // set current to the parent of the incomming node
                // if last_header_numbers - header_numbers +1 == 0;
                //     cause 0..0 = 0 iteration
                // then current = incomming node's parent
                // so skip this step
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
            current.borrow_mut().author = Some((0, author.to_owned()));

            author.clear();

            last_header_numbers = header_numbers;
            header_numbers = 0;

            except = false;
            author_phase = false;
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
        } else if *c == '\n' && condition_phase && dialog_type.is_choice()
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

            let choice: DialogType = match (condition.karma_threshold, condition.clone().event) {
                (None, None) => DialogType::Choice {
                    text: save.clone(),
                    condition: None,
                },
                (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => DialogType::Choice {
                    text: save.clone(),
                    condition: Some(condition.clone()),
                },
            };

            current.borrow_mut().dialog_type.push(choice);

            condition_phase = false;

            save.clear();
            condition = DialogCondition::default();
        } else if *c == '\n'
            && content_phase
            && !save.is_empty()
            && dialog_type.is_text()
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

            let dialog = DialogType::Text(save.clone());
            current.borrow_mut().dialog_type.push(dialog);

            save.clear();

            content_phase = false;
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
            else if condition_phase {
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

                dialog_type = DialogType::new_text();

                author_phase = false;
                content_phase = false;
                // useless protection cause by if condition_phase just above
                // condition_phase = false;
            } else if author_phase && *c != '#' {
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
