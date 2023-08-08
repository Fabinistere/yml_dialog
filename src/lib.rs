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

use log::warn;
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

// REFACTOR: Merge WorldEvent and TriggerEvent

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

/// Choice's Condition
///
/// # Note
///
/// NOTE: CUSTOM
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DialogCondition {
    karma_threshold: Option<(i32, i32)>,
    events: Option<Vec<String>>,
}

impl DialogCondition {
    /// Construct a new DialogCondition with the given `karma_threshold` and `event`.
    pub fn new(
        karma_threshold: Option<(i32, i32)>,
        events: Option<Vec<String>>,
    ) -> DialogCondition {
        DialogCondition {
            karma_threshold,
            events,
        }
    }

    /// Only check if the given `karma` is within the range of the condition
    ///
    /// # Note
    ///
    /// NOTE: Could be depreciated to only use `is_verified`
    pub fn is_karma_verified(&self, karma: i32) -> bool {
        match self.karma_threshold {
            None => true,
            Some(karma_threshold) => karma >= karma_threshold.0 && karma <= karma_threshold.1,
        }
    }

    /// Only check if the condition's event is contained in the given `events`
    ///
    /// # Note
    ///
    /// NOTE: Could be depreciated to only use `is_verified`
    pub fn is_events_verified(&self, events: Vec<String>) -> bool {
        match self.events() {
            None => true,
            Some(events_to_have) => {
                let mut all_contained = true;
                for event in events_to_have {
                    if !events.contains(event) {
                        all_contained = false;
                        break;
                    }
                }
                all_contained
            }
        }
    }

    /// Verify a Choice's condition
    pub fn is_verified(&self, karma: Option<i32>, events: Option<Vec<String>>) -> bool {
        let karma_verified = match karma {
            None => self.karma_threshold == None,
            Some(karma) => self.is_karma_verified(karma),
        };
        let events_verified = match events {
            None => self.events() == &None,
            Some(events) => self.is_events_verified(events),
        };

        karma_verified && events_verified
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
    pub fn events(&self) -> &Option<Vec<String>> {
        &self.events
    }

    /// Returns the mtable reference `event` item of the `DialogCondition`.
    pub fn events_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.events
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
    dialog_content: Vec<DialogContent>,
    /// The potential name of the speaker.
    author: Option<String>,
    /// Vector of its subtree.
    children: Vec<Rc<RefCell<DialogNode>>>,
    /// Its potential Parent.
    ///
    /// # Note
    ///
    /// maybe too much (prefer a stack in the TreeIterator)
    parent: Option<Rc<RefCell<DialogNode>>>,
    /// All events triggered by this dialogue.
    ///
    /// # Note
    ///
    /// NOTE: CUSTOM (but how...)
    trigger_event: Vec<String>,
}

impl DialogNode {
    /// COnstructs a new DialogNode with the given
    /// - `content`,
    /// - `author`,
    /// - `children`,
    /// - `parent`
    /// - `trigger_event`
    pub fn new(
        content: Vec<DialogContent>,
        author: Option<String>,
        children: Vec<Rc<RefCell<DialogNode>>>,
        parent: Option<Rc<RefCell<DialogNode>>>,
        trigger_event: Vec<String>,
    ) -> Self {
        DialogNode {
            dialog_content: content,
            author,
            children,
            parent,
            trigger_event,
        }
    }
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
    ///
    /// ```rust
    /// let answers = DialogNode::new(
    ///     vec![
    ///         DialogContent::Choice {
    ///             text: String::from("Hello"),
    ///             condition: None,
    ///         },
    ///         DialogContent::Choice {
    ///             text: String::from("No Hello"),
    ///             condition: None,
    ///         },
    ///     ],
    ///     Some("You".to_string()),
    ///     Vec::new(),
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// assert!(answers.is_choice())
    /// ```
    pub fn is_choice(&self) -> bool {
        match self.dialog_content.first() {
            None => false,
            Some(first) => first.is_choice(),
        }
    }

    /// # Return
    ///
    /// `true` if the type of the first element (of dialog_content) is choice
    ///
    /// ```rust
    /// let catchphrase = DialogNode::new(
    ///     vec![
    ///         DialogContent::Text(String::from("Hello")),
    ///         DialogContent::Text(String::from("How are you?")),
    ///     ],
    ///     Some("You".to_string()),
    ///     Vec::new(),
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// assert!(answers.is_choice())
    /// ```
    pub fn is_text(&self) -> bool {
        match self.dialog_content.first() {
            None => false,
            Some(first) => first.is_text(),
        }
    }

    /// Put `new_node` in the end of the children vector.
    pub fn add_child(&mut self, new_node: Rc<RefCell<DialogNode>>) {
        self.children.push(new_node);
    }

    /// TEMP: If the children is a choice and if there is at least one verified choice
    ///
    /// ```rust
    /// # use std::{rc::Rc, cell::RefCell};
    /// # use fto_dialog::{DialogNode, DialogContent};
    /// let answers = DialogNode::new(
    ///     vec![
    ///         DialogContent::Choice {
    ///             text: String::from("Hello"),
    ///             condition: None,
    ///         },
    ///         DialogContent::Choice {
    ///             text: String::from("No Hello"),
    ///             condition: None,
    ///         },
    ///     ],
    ///     Some("You".to_string()),
    ///     Vec::new(),
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// let catchphrase = DialogNode::new(
    ///     vec![
    ///         DialogContent::Text(String::from("Hello")),
    ///     ],
    ///     Some("You".to_string()),
    ///     vec![Rc::new(RefCell::new(answers))],
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// assert!(catchphrase.at_least_one_child_is_verified(None, None))
    /// ```
    ///
    /// ```rust
    /// # use std::{rc::Rc, cell::RefCell};
    /// # use fto_dialog::{DialogNode, DialogContent};
    /// let answer = DialogNode::new(
    ///     vec![
    ///         DialogContent::Text(String::from("No, you're cool"))
    ///     ],
    ///     Some("Not You".to_string()),
    ///     Vec::new(),
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// let catchphrase = DialogNode::new(
    ///     vec![
    ///         DialogContent::Text(String::from("Hello CoolFolk")),
    ///     ],
    ///     Some("You".to_string()),
    ///     vec![Rc::new(RefCell::new(answer))],
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// assert!(catchphrase.at_least_one_child_is_verified(None, None))
    /// ```
    ///
    /// ```rust
    /// # use std::{rc::Rc, cell::RefCell};
    /// # use fto_dialog::{DialogNode, DialogContent, DialogCondition};
    /// let answers = DialogNode::new(
    ///     vec![
    ///         DialogContent::Choice {
    ///             text: String::from("Hello"),
    ///             condition: Some(DialogCondition::new(Some((50, 100)), None)),
    ///         },
    ///         DialogContent::Choice {
    ///             text: String::from("No Hello"),
    ///             condition: Some(DialogCondition::new(Some((-50, 0)), None)),
    ///         },
    ///     ],
    ///     Some("You".to_string()),
    ///     Vec::new(),
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// let catchphrase = DialogNode::new(
    ///     vec![
    ///         DialogContent::Text(String::from("Hello")),
    ///     ],
    ///     Some("You".to_string()),
    ///     vec![Rc::new(RefCell::new(answers))],
    ///     None,
    ///     Vec::new(),
    /// );
    ///
    /// assert!(!catchphrase.at_least_one_child_is_verified(Some(20), None))
    /// ```
    pub fn at_least_one_child_is_verified(
        &self,
        karma: Option<i32>,
        events: Option<Vec<String>>,
    ) -> bool {
        for child in &self.children {
            for content in child.borrow().content() {
                match content {
                    DialogContent::Text(_) => return true,
                    DialogContent::Choice { text: _, condition } => {
                        return match condition {
                            None => true,
                            Some(condition) => condition.is_verified(karma, events),
                        };
                    }
                }
            }
        }
        false
    }

    /// Give the read-only author of the node.
    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    /// Give the mutable author of the node.
    pub fn author_mut(&mut self) -> &mut Option<String> {
        &mut self.author
    }

    /// Give the read-only content of the node.
    pub fn content(&self) -> &Vec<DialogContent> {
        &self.dialog_content
    }

    /// Give the mutable content of the node.
    pub fn content_mut(&mut self) -> &mut Vec<DialogContent> {
        &mut self.dialog_content
    }

    /// Give the read-only `children` of the node.
    pub fn children(&self) -> &Vec<Rc<RefCell<DialogNode>>> {
        &self.children
    }

    /// Give the mutable children of the node.
    pub fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<DialogNode>>> {
        &mut self.children
    }

    /// Give the read-only `parent` of the node.
    pub fn parent(&self) -> &Option<Rc<RefCell<DialogNode>>> {
        &self.parent
    }

    /// Give the mutable parent of the node.
    pub fn parent_mut(&mut self) -> &mut Option<Rc<RefCell<DialogNode>>> {
        &mut self.parent
    }

    /// Give the read-only `trigger_event` of the node.
    pub fn trigger_event(&self) -> &Vec<String> {
        &self.trigger_event
    }

    /// Give the mutable trigger_event of the node.
    pub fn trigger_event_mut(&mut self) -> &mut Vec<String> {
        &mut self.trigger_event
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
    /// - This text will be prompted only if the fourth choice is selected by the player\n
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

                        match &dialog_condition.events() {
                            Some(events) => {
                                if !events.is_empty() {
                                    // DOC: events in plurial ?
                                    res.push_str("event: ");
                                    for event in events {
                                        res.push_str(event);
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

        if !self.trigger_event.is_empty() {
            res.push_str("\n->");
        }
        for trigger_event in &self.trigger_event {
            res.push_str(&format!(" {},END", trigger_event));
        }
        if !self.trigger_event.is_empty() {
            res.push('\n');

            res = res.replace(",END ", ", ");
            res = res.replace(",END\n", "\n");
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

        /* -------------------------------------------------------------------------- */
        /*                           Remove the last "\n\n"                           */
        /* -------------------------------------------------------------------------- */
        res = res.replace("END#", "#");
        // keep only one last '\n' (to respect init rules)
        res = res.replace("\n\nEND", "\n");

        res
    }
}

/// It's an argument of `init_tree_file`
/// used to get custom infos from the user.
///
/// # Note
///  
/// TODO: Create setter (getter is kinda useless cause we're using it here)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DialogCustomInfos {
    /// Set of event that could be required to
    /// unlock a Choice, in `DialogCondition`.
    ///
    /// # Tips
    ///
    /// - You can define an enum with all your `WorldEvent`
    /// and iter through all of them and "print" them in.
    ///
    /// ```rust
    /// use enum_iter::EnumIter;
    ///
    /// /// World event
    /// #[derive(EnumIter, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    /// pub enum GameEvent {
    ///     BeatTheGame,
    ///     FirstKill,
    ///     AreaCleared,
    ///     HasCharisma,
    ///     HasFriend,
    /// }
    ///
    /// impl fmt::Display for GameEvent {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         match self {
    ///             GameEvent::BeatTheGame => write!(f, "BeatTheGame"),
    ///             GameEvent::FirstKill => write!(f, "FirstKill"),
    ///             GameEvent::AreaCleared => write!(f, "AreaCleared"),
    ///             GameEvent::HasCharisma => write!(f, "HasCharisma"),
    ///             GameEvent::HasFriend => write!(f, "HasFriend"),
    ///         }
    ///     }
    /// }
    ///
    /// impl FromStr for GameEvent {
    ///     type Err = ();
    ///
    ///     fn from_str(input: &str) -> Result<GameEvent, Self::Err> {
    ///         match input {
    ///             "BeatTheGame" => Ok(GameEvent::BeatTheGame),
    ///             "FirstKill" => Ok(GameEvent::FirstKill),
    ///             "AreaCleared" => Ok(GameEvent::AreaCleared),
    ///             "HasCharisma" => Ok(GameEvent::HasCharisma),
    ///             "HasFriend" => Ok(GameEvent::HasFriend),
    ///             _ => Err(()),
    ///         }
    ///     }
    /// }
    ///
    /// TODO: Iter throw the enum and put it in this field
    /// let infos = DialogCustomInfos::new(
    ///     GameEvent::iter()
    ///         .map(|x| x.to_string())
    ///         .collect::<Vec<String>>(),
    ///     Some((-100, 100)),
    ///     GameEvent::iter()
    ///         .map(|x| x.to_string())
    ///         .collect::<Vec<String>>(),
    /// );
    /// let s = String::from("# Player\n\n- I beat the game | e: BeatTheGame;\n- There is no hope | None\n");
    /// let tree = init_tree_file(s, infos);
    /// ```
    world_event: Vec<String>,
    /// Define the (Min, Max) for the KarmaThreshold of `DialogConditon`
    /// to allow the user to write Max instead of a given number
    karma_limits: Option<(i32, i32)>,
    /// Set of event that could be triggered when leaving/entering (your implementation choice)
    /// a node.
    trigger_event: Vec<String>,
}

impl DialogCustomInfos {
    // /// Constructs a new empty `DialogCustomInfos`
    // pub fn new() -> Self {
    //     DialogCustomInfos::default()
    // }

    /// Constructs a new `DialogCustomInfos` with the given `world_event`, `karma_limits` and `trigger_event`
    pub fn new(
        world_event: Vec<String>,
        karma_limits: Option<(i32, i32)>,
        trigger_event: Vec<String>,
    ) -> Self {
        DialogCustomInfos {
            world_event,
            karma_limits,
            trigger_event,
        }
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
/// # use std::{cell::RefCell, rc::Rc};
/// # use fto_dialog::DialogNode;
/// use fto_dialog::{init_tree_file, DialogCustomInfos};
///
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
///     ),
///     DialogCustomInfos::new(
///         Vec::from([String::from("OlfBeaten")]),
///         None,
///         Vec::from([String::from("OpenTheHospis")]),
///     ),
/// );
/// ```
///
/// To create a long monologue,
/// avoid using `\n`, prefere using `-`
/// to seperated paragraph
///
/// ```rust
/// # use std::{cell::RefCell, rc::Rc};
/// # use fto_dialog::DialogNode;
/// use fto_dialog::{init_tree_file, DialogCustomInfos};
///
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
///     ),
///     DialogCustomInfos::new(
///         Vec::from([String::from("WonTheLottery")]),
///         Some((0, 100)),
///         Vec::from([String::from("FightEvent")]),
///     ),
/// );
/// ```
///
/// [DialogTree format]: #rules
pub fn init_tree_file(s: String, infos: DialogCustomInfos) -> Rc<RefCell<DialogNode>> {
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
    // DEBUG: println!("{}", chars.len());
    // DEBUG: println!("{}", s);

    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < chars.len())
    {
        // DEBUG: Print working symbol
        // println!(
        //     "c: {}, {:?}, event_phase: {event_phase}, karma_phase: {karma_phase}",
        //     *c, phase
        // );

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
            // DEBUG: println!("karma 1rst elem: {}", karma);
            let k: i32 = if karma == *"MAX" || karma == *"max" {
                match infos.karma_limits {
                    // TODO: Send Err
                    None => {
                        warn!("This `karma_limits` is not definded in `infos`");
                        0
                    }
                    Some((_, max)) => max,
                }
            } else if karma == *"MIN" || karma == *"min" {
                match infos.karma_limits {
                    // TODO: Send Err
                    None => {
                        warn!("This `karma_limits` is not definded in `infos`");
                        0
                    }
                    Some((min, _)) => min,
                }
            } else {
                karma.parse::<i32>().unwrap()
            };
            condition.karma_threshold = match infos.karma_limits {
                None => Some((k, 0)),
                Some((_, max)) => Some((k, max)),
            };

            karma.clear();
        }
        /* -------------------------------------------------------------------------- */
        /*                                End of Phase                                */
        /* -------------------------------------------------------------------------- */
        else if *c == '\n' && phase == Some(InitPhase::AuthorPhase) {
            while author.starts_with(' ') {
                author.remove(0);
            }
            // DEBUG: println!("author: {}", author);

            // root: header_numbers != 1
            if last_header_numbers != 0 {
                // DEBUG: println!("previous:\n{}", current.borrow().print_file());
                // DEBUG: println!("last_header_numbers {}", last_header_numbers);
                // DEBUG: println!("header_numbers {}", header_numbers);

                /* set current to the parent of the incomming node
                 * if last_header_numbers - header_numbers +1 == 0;
                 *     cause 0..0 = 0 iteration
                 * then current = incomming node's parent
                 * so skip this step
                 */
                let limit = last_header_numbers - header_numbers + 1;
                for _step in 0..limit {
                    // DEBUG: println!("step {}", _step);
                    // should not panic anyway
                    let parent = current.clone().borrow().parent.clone();
                    // DEBUG: println!(
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
            // DEBUG: println!("karma: {}", karma);
            let k: i32 = if karma == *"MAX" || karma == *"max" {
                match infos.karma_limits {
                    // TODO: Send Err
                    None => {
                        warn!("This `karma_limits` is not definded in `infos`");
                        0
                    }
                    Some((_, max)) => max,
                }
            } else if karma == *"MIN" || karma == *"min" {
                match infos.karma_limits {
                    // TODO: Send Err
                    None => {
                        warn!("This `karma_limits` is not definded in `infos`");
                        0
                    }
                    Some((min, _)) => min,
                }
            } else {
                karma.parse::<i32>().unwrap()
            };

            match condition.karma_threshold {
                None => {
                    warn!("init creation condition has been skipped");
                }
                // `_` cause the second `i32` is a placeholder
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
            // DEBUG: println!("event : {}", event);
            if infos.world_event.contains(&event) {
                match condition.events_mut() {
                    Some(ref mut vec) => {
                        vec.push(event.clone());
                    }
                    None => *condition.events_mut() = Some(vec![event.clone()]),
                }
            } else {
                // TODO: Send Err
                warn!("This `WorldEvent` is not definded in `infos`");
            }

            // DEBUG: println!("event cleared");
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
            // remove all spaces from edge positions
            while save.starts_with(' ') {
                save.remove(0);
            }
            while save.ends_with(' ') {
                save.remove(save.len() - 1);
            }

            // DEBUG: println!("choice inserted: {}", save);

            let choice: DialogContent =
                match (condition.karma_threshold(), condition.clone().events()) {
                    (None, None) => DialogContent::Choice {
                        text: save.clone(),
                        condition: None,
                    },
                    (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                        DialogContent::Choice {
                            text: save.clone(),
                            condition: Some(condition.clone()),
                        }
                    }
                };

            current.borrow_mut().dialog_content.push(choice);

            phase = None;
            karma_phase = false;
            event_phase = false;

            save.clear();
            condition = DialogCondition::default();
        } else if *c == '\n'
            && phase == Some(InitPhase::ContentPhase)
            && !save.is_empty()
            && dialog_content.is_text()
            && !except
        {
            // remove all spaces from edge positions
            while save.starts_with(' ') {
                save.remove(0);
            }
            while save.ends_with(' ') {
                save.remove(save.len() - 1);
            }

            // DEBUG: println!("text inserted: {}", save);

            let dialog = DialogContent::Text(save.clone());
            current.borrow_mut().dialog_content.push(dialog);

            save.clear();

            phase = None;
        } else if (*c == ',' || *c == '\n') && trigger_phase {
            // DEBUG: println!("event : {}", event);
            if infos.trigger_event.contains(&event) {
                current.borrow_mut().trigger_event.push(event.clone());
            } else {
                // TODO: Send Err
                warn!("This `TriggerEvent` is not definded in `infos`");
            }

            // DEBUG: println!("event cleared");
            event.clear();

            if *c == '\n' {
                trigger_phase = false;
            }
        }
        /* -------------------------------------------------------------------------- */
        /*                              Text or Condition                             */
        /* -------------------------------------------------------------------------- */
        else if c.is_ascii() || c.is_alphanumeric() {
            // the except char is `/`
            if *c == '/' {
                // DEBUG: info!("except");
                except = true;
            }
            // `;` or `\n` put an end to the selection of condtion
            else if phase == Some(InitPhase::ConditionPhase) {
                if *c == 'k' && !event_phase {
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

                    // DEBUG: println!("k: {}", karma);
                } else if event_phase && *c != ' ' && !post_colon {
                    event.push(*c);

                    // DEBUG: println!("e: {}", event);
                }
            } else if trigger_phase && *c != ' ' {
                event.push(*c);
            }
            // ignore the new line: "\n"
            // the \n is a marker to end some phase
            else if *c == '\n' && !except {
                // new_line = true;
                // DEBUG: println!("skip");

                // full reset

                dialog_content = DialogContent::new_text();

                phase = None;
            } else if phase == Some(InitPhase::AuthorPhase) && *c != '#' {
                author.push(*c);
            }
            // else if !is_special_char_file(*c) || (is_special_char_file(*c) && except)
            else {
                save.push(*c);
                except = false;

                // DEBUG: println!("add {} ", *c);
            }
        }
    }

    root
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DialogNodeYML {
    source: String,
    content: Content,
    trigger_event: Vec<String>,
}

impl DialogNodeYML {
    pub fn new(source: String, content: Content, trigger_event: Vec<String>) -> Self {
        DialogNodeYML {
            source,
            content,
            trigger_event,
        }
    }
}

/// TODO: Custom impl Serialization
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Content {
    Choices(Vec<Choice>),
    Monolog {
        text: Vec<String>,
        exit_state: usize,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Choice {
    text: String,
    /// TODO: Custom Generic Type
    condition: Option<String>,
    /// if the `exit_state` is not a key from the Map, it's a end node.
    exit_state: usize,
}

impl Choice {
    pub fn new(text: String, condition: Option<String>, exit_state: usize) -> Self {
        Choice {
            text,
            condition,
            exit_state,
        }
    }
}

// impl Serialize for Choice {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("Choice", 3)?;
//         state.serialize_field("text", &self.text)?;
//         state.serialize_field("condition", &self.condition)?;
//         state.serialize_field("exit_state", &self.exit_state)?;
//         state.end()
//     }
// }

/// The generic Type `C` corresponds to your custom struct Condition.
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// fn main() -> Result<(), serde_yaml::Error> {
///     // You have some type.
///     let mut map = BTreeMap::new();
///     map.insert("x".to_string(), 1.0);
///     map.insert("y".to_string(), 2.0);
///
///     // Serialize it to a YAML string.
///     let yaml = serde_yaml::to_string(&map)?;
///     assert_eq!(yaml, "x: 1.0\ny: 2.0\n");
///
///     // Deserialize it back to a Rust type.
///     let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&yaml)?;
///     assert_eq!(map, deserialized_map);
///     Ok(())
/// }
/// ```
pub fn dialog_deserializer(
    dialog_file: String,
) -> Result<BTreeMap<usize, DialogNodeYML>, serde_yaml::Error> {
    let deserialized_map: BTreeMap<usize, DialogNodeYML> = serde_yaml::from_str(&dialog_file)?;
    Ok(deserialized_map)
}
