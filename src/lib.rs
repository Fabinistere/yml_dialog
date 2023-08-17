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
#![warn(missing_docs)]

use log::warn;
use serde::{
    // ser::{SerializeStruct, SerializeStructVariant, Serializer},
    Deserialize,
    Serialize,
};

/// This correspond to a unique key
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[serde(default)]
pub struct DialogNode {
    source: String,
    content: Content,
    /// REFACTOR: Turn this into a generic type `extra`
    trigger_event: Vec<String>,
}

impl DialogNode {
    /// Constructs a new DialogNode with the given
    /// - `source`,
    /// - `content`,
    /// - `trigger_event` vector
    pub fn new(source: String, content: Content, trigger_event: Vec<String>) -> Self {
        DialogNode {
            source,
            content,
            trigger_event,
        }
    }

    /// Give the read-only author of the node.
    pub fn source(&self) -> &String {
        &self.source
    }

    /// Give the mutable author of the node.
    pub fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }

    /// Give the read-only `content` of the node.
    pub fn content(&self) -> &Content {
        &self.content
    }

    /// Give the mutable `content` of the node.
    pub fn content_mut(&mut self) -> &mut Content {
        &mut self.content
    }

    /// Give the read-only `trigger_event` of the node.
    pub fn trigger_event(&self) -> &Vec<String> {
        &self.trigger_event
    }

    /// Give the mutable trigger_event of the node.
    pub fn trigger_event_mut(&mut self) -> &mut Vec<String> {
        &mut self.trigger_event
    }
}

/// A Node is either a list of choice or a monolog
///
/// ```rust
/// use yml_dialog::*;
/// use std::collections::BTreeMap;
///
/// let mut map = BTreeMap::new();
/// map.insert(
///     1,
///     DialogNode::new(
///         "The Frog".to_string(),
///         Content::Choices(vec![
///             Choice::new(String::from("Hello HomeGirl"), None, 2),
///             Choice::new(String::from("KeroKero"), None, 3),
///         ]),
///         vec![],
///     ),
/// );    
/// map.insert(
///     2,
///     DialogNode::new(
///         "Random Frog".to_string(),
///         Content::Monolog {
///             text: vec![String::from("Yo Homie")],
///             exit_state: 4,
///         },
///         vec![],
///     ),
/// );
/// map.insert(
///     3,
///     DialogNode::new(
///         "Random Frog".to_string(),
///         Content::Monolog {
///             text: vec![String::from("KeroKero")],
///             exit_state: 4,
///         },
///         vec![],
///     ),
/// );
/// let yaml = serde_yaml::to_string(&map).unwrap();
///
/// assert_eq!(
///     yaml,
///     "1:
///   source: The Frog
///   content:
///   - text: Hello HomeGirl
///     condition: null
///     exit_state: 2
///   - text: KeroKero
///     condition: null
///     exit_state: 3
///   trigger_event: []
/// 2:
///   source: Random Frog
///   content:
///     text:
///     - Yo Homie
///     exit_state: 4
///   trigger_event: []
/// 3:
///   source: Random Frog
///   content:
///     text:
///     - KeroKero
///     exit_state: 4
///   trigger_event: []\n"
///         .to_string()
///     )
/// ```
///
/// # Note
///
/// TODO: Custom impl Serialization
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(untagged)]
pub enum Content {
    /// A vector of Choice
    Choices(Vec<Choice>),
    /// A text block, containing a vector of text line
    /// and an exit state
    Monolog {
        /// The vector of text line
        text: Vec<String>,
        /// The exit state
        exit_state: usize,
    },
}

impl Default for Content {
    fn default() -> Self {
        Content::Monolog {
            text: vec![],
            exit_state: 0,
        }
    }
}

// impl Serialize for Content {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Content::Choices(choices) => {
//                 let state = serializer.serialize_newtype_variant("", 0, "choices", choices)?;
//                 Ok(state)
//                 // state.end()
//             }
//             Content::Monolog { text, exit_state } => {
//                 let mut state = serializer.serialize_struct_variant("", 1, "monolog", 2)?;
//                 state.serialize_field("text", text)?;
//                 state.serialize_field("state", exit_state)?;
//                 state.end()
//             }
//         }
//     }
// }

/// A Choice is composed of
/// - a `text` line,
/// - a `condition` and
/// - an `exit_state` corresponding to the continue of this choice.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[serde(default)]
pub struct Choice {
    text: String,
    /// REFACTOR: Custom Generic Type
    condition: Option<Condition>,
    /// if the `exit_state` is not a key from the Map, it's a end node.
    exit_state: usize,
}

impl Choice {
    /// Constructs a new Choice with the given `text`, `condition`, `exit_state`,
    pub fn new(text: String, condition: Option<Condition>, exit_state: usize) -> Self {
        Choice {
            text,
            condition,
            exit_state,
        }
    }

    /// Retruns the bool: The choice's condition is verified with the given `karma` and the set of `active_event`
    pub fn is_verified(&self, karma: Option<i32>, active_events: Vec<String>) -> bool {
        match &self.condition {
            None => self.condition.is_none(),
            Some(condition) => condition.is_verified(karma, active_events),
        }
    }

    /// Returns the read-only `text` of the choice
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Returns the mutable `text` of the choice
    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    /// Returns the read-only `condition` of the choice
    pub fn condition(&self) -> &Option<Condition> {
        &self.condition
    }

    /// Returns the mutable `condition` of the choice
    pub fn condition_mut(&mut self) -> &mut Option<Condition> {
        &mut self.condition
    }

    /// Returns the read-only `exit_state` of the choice
    pub fn exit_state(&self) -> &usize {
        &self.exit_state
    }

    /// Returns the mutable `exit_state` of the choice
    pub fn exit_state_mut(&mut self) -> &mut usize {
        &mut self.exit_state
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

/// REFACTOR: Turn this into a Generic Type
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Condition {
    events: Vec<String>,
    karma_threshold: Option<(i32, i32)>,
}

impl Condition {
    /// Construct a new Condition with the given `karma_threshold` and `event`.
    pub fn new(karma_threshold: Option<(i32, i32)>, events: Vec<String>) -> Condition {
        Condition {
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
    pub fn is_events_verified(&self, active_events: Vec<String>) -> bool {
        let mut all_contained = true;
        for event in self.events() {
            if !active_events.contains(event) {
                all_contained = false;
                break;
            }
        }
        all_contained
    }

    /// Verify a Choice's condition with the potential tested karma and the set of active event
    pub fn is_verified(&self, karma: Option<i32>, active_events: Vec<String>) -> bool {
        let karma_verified = match karma {
            None => self.karma_threshold.is_none(),
            Some(karma) => self.is_karma_verified(karma),
        };
        let events_verified = self.is_events_verified(active_events);

        karma_verified && events_verified
    }

    /// Returns the read-only `karma_threshold` item of the `Condition`.
    pub fn karma_threshold(&self) -> &Option<(i32, i32)> {
        &self.karma_threshold
    }

    /// Returns the mutable reference `karma_threshold` item of the `Condition`.
    pub fn karma_threshold_mut(&mut self) -> &mut Option<(i32, i32)> {
        &mut self.karma_threshold
    }

    /// Returns the read-only `event` item of the `Condition`.
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }

    /// Returns the mtable reference `event` item of the `Condition`.
    pub fn events_mut(&mut self) -> &Vec<String> {
        &mut self.events
    }
}
