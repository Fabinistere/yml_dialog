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
    ser::{SerializeStruct, SerializeStructVariant, Serializer},
    Deserialize, Serialize,
};
use std::collections::BTreeMap;

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
