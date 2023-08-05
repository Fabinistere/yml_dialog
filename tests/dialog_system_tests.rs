#[cfg(test)]
use std::{cell::RefCell, fmt, rc::Rc, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use fto_dialog::*;

const KARMA_MAX: i32 = 100;
const KARMA_MIN: i32 = -KARMA_MAX;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter)]
enum WorldEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
    HasCharisma,
    HasFriend,
    TalkWith1000Frog,
}

impl fmt::Display for WorldEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorldEvent::BeatTheGame => write!(f, "BeatTheGame"),
            WorldEvent::FirstKill => write!(f, "FirstKill"),
            WorldEvent::AreaCleared => write!(f, "AreaCleared"),
            WorldEvent::HasCharisma => write!(f, "HasCharisma"),
            WorldEvent::HasFriend => write!(f, "HasFriend"),
            WorldEvent::TalkWith1000Frog => write!(f, "TalkWith1000Frog"),
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
            "TalkWith1000Frog" => Ok(WorldEvent::TalkWith1000Frog),
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

#[test]
fn test_print_from_file() {
    let fabien = Some(String::from("Fabien"));
    let morgan = Some(String::from("Morgan"));

    let dialog = Rc::new(RefCell::new(DialogNode::new(
        vec![DialogContent::Text(String::from("Hello"))],
        fabien.clone(),
        Vec::new(),
        None,
        Vec::new(),
    )));

    let answers = Rc::new(RefCell::new(DialogNode::new(
        vec![
            DialogContent::Choice {
                text: String::from("Hey"),
                condition: None,
            },
            DialogContent::Choice {
                text: String::from("No Hello"),
                condition: None,
            },
            DialogContent::Choice {
                text: String::from("Want to share a flat ?"),
                condition: None,
            },
        ],
        morgan,
        Vec::new(),
        None,
        Vec::new(),
    )));

    let dialog_2 = Rc::new(RefCell::new(DialogNode::new(
        vec![DialogContent::Text(String::from(":)"))],
        fabien.clone(),
        Vec::new(),
        None,
        Vec::new(),
    )));

    let dialog_3 = Rc::new(RefCell::new(DialogNode::new(
        vec![DialogContent::Text(String::from(":O"))],
        fabien.clone(),
        Vec::new(),
        None,
        Vec::new(),
    )));

    let dialog_4 = Rc::new(RefCell::new(DialogNode::new(
        vec![DialogContent::Text(String::from("Sure"))],
        fabien.clone(),
        Vec::new(),
        None,
        Vec::new(),
    )));

    answers.borrow_mut().add_child(dialog_2);
    answers.borrow_mut().add_child(dialog_3);
    answers.borrow_mut().add_child(dialog_4);

    dialog.borrow_mut().add_child(answers);

    assert_eq!(
        dialog.borrow().print_file(),
        "# Fabien

- Hello

## Morgan

- Hey | None
- No Hello | None
- Want to share a flat ? | None

### Fabien

- :)

### Fabien

- :O

### Fabien

- Sure\n"
            .to_string()
    );
}

#[test]
fn test_print_from_file_monologue() {
    let root = init_tree_file(
        String::from("# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(
        root.borrow().print_file(),
        "# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n".to_string()
    );
}

#[test]
fn test_print_from_file_trigger_event_1() {
    let root = init_tree_file(
        String::from("# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n\n-> HasFriend\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(
        root.borrow().print_file(),
        "# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n\n-> HasFriend\n".to_string()
    );
}

#[test]
fn test_print_from_file_trigger_event_2() {
    let root = init_tree_file(
        String::from("# Olf\n\n- Hello\n\n-> HasFriend, FightEvent\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(
        root.borrow().print_file(),
        "# Olf\n\n- Hello\n\n-> HasFriend, FightEvent\n".to_string()
    );
}

#[test]
fn test_init_tree_from_file_simple_text_1() {
    let root = init_tree_file(
        String::from("# Olf\n\n- Hello\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Olf")));

    assert_eq!(
        *root.borrow().content(),
        vec![DialogContent::Text("Hello".to_string())]
    );
}

#[test]
fn test_init_tree_from_file_space_overdose_1() {
    let root = init_tree_file(
        String::from("#            Olf\n\n-      Hello\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Olf")));

    assert_eq!(
        *root.borrow().content(),
        vec![DialogContent::Text("Hello".to_string())]
    );
}

#[test]
fn test_init_tree_from_file_space_overdose_2() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello         |   None\n- No Hello    | None\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: None
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_space_deficiency_1() {
    let root = init_tree_file(
        String::from("#Olf\n\n-Hello\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Olf")));

    assert_eq!(
        *root.borrow().content(),
        vec![DialogContent::Text("Hello".to_string())]
    );
}

#[test]
fn test_init_tree_from_file_space_deficiency_2() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello|None\n- No Hello|None\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: None
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_monologue_1() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello\n- I was wondering\n-Alone...\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Text("Hello".to_string()),
            DialogContent::Text("I was wondering".to_string()),
            DialogContent::Text("Alone...".to_string())
        ]
    );
}

#[test]
fn test_init_tree_from_file_simple_choice_1() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello | None\n- No Hello | None\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: None
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_complex_choice_1() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello | None\n- No Hello | k: -10,0;\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: Some(DialogCondition::new(Some((-10, 0)), None))
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_complex_choice_2() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello | None\n- Mary me Hugo. | e: HasCharisma;\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "Mary me Hugo.".to_string(),
                condition: Some(DialogCondition::new(
                    None,
                    Some(vec![WorldEvent::HasCharisma.to_string()])
                ))
            }
        ]
    );
}

// allow to explicit/implicit karma/k; event/e
// extend the tolerance

#[test]
fn test_init_tree_from_file_complex_choice_3() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello | k: -50,100;\n- No Hello | karma : -100,0;\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: Some(DialogCondition::new(Some((-50, 100)), None,))
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: Some(DialogCondition::new(Some((-100, 0)), None,))
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_complex_choice_4() {
    let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello my Friend | e: HasFriend;\n- You droped this (*crown*) | event: HasCharisma;\n",
            ), DialogCustomInfos::new(
                WorldEvent::iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                Some((KARMA_MIN, KARMA_MAX)),
                TriggerEvent::iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
    ));

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello my Friend".to_string(),
                condition: Some(DialogCondition::new(
                    None,
                    Some(vec![WorldEvent::HasFriend.to_string()])
                ))
            },
            DialogContent::Choice {
                text: "You droped this (*crown*)".to_string(),
                condition: Some(DialogCondition::new(
                    None,
                    Some(vec![WorldEvent::HasCharisma.to_string()])
                ))
            }
        ]
    );
}

/// Verify the case of a event has a k in its name work.
/// Used to trigger the phase transi into the the karma_phase
#[test]
fn test_init_tree_from_file_complex_choice_event_with_k() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- KeroKero | e: TalkWith1000Frog;\n- Hello | None;\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "KeroKero".to_string(),
                condition: Some(DialogCondition::new(
                    None,
                    Some(vec!["TalkWith1000Frog".to_string()])
                ))
            },
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            }
        ]
    );
}

/// allow to type MAX or MIN to select
#[test]
fn test_init_tree_from_file_complex_choice_karma_max_min() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello | k: -10,MAX;\n- No Hello | k: MIN,0;\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: Some(DialogCondition::new(Some((-10, KARMA_MAX)), None))
            },
            DialogContent::Choice {
                text: "No Hello".to_string(),
                condition: Some(DialogCondition::new(Some((KARMA_MIN, 0)), None))
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_simple_kinship_1() {
    let root = init_tree_file(
        String::from("# Morgan\n\n- Hello\n## Hugo\n- Hey! How are you ?\n"),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));
    assert_eq!(
        *root.borrow().children()[0].borrow().author(),
        Some(String::from("Hugo"))
    );

    assert_eq!(
        *root.borrow().content(),
        vec![DialogContent::Text("Hello".to_string())]
    );
    assert_eq!(
        *root.borrow().children()[0].borrow().content(),
        vec![DialogContent::Text("Hey! How are you ?".to_string())]
    );
}

#[test]
fn test_init_tree_from_file_monologue_2() {
    let root = init_tree_file(String::from("# Morgan\n\n- Hello\n- I was wondering\n\n## Morgan\n\n- With Friends ! | event: HasFriend;\n- Alone... | None\n"), DialogCustomInfos::new(
    WorldEvent::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(),
    Some((KARMA_MIN, KARMA_MAX)),
    TriggerEvent::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(),
));

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Text("Hello".to_string()),
            DialogContent::Text("I was wondering".to_string())
        ]
    );

    assert_eq!(
        *root.borrow().children()[0].borrow().author(),
        Some(String::from("Morgan"))
    );

    assert_eq!(
        *root.borrow().children()[0].borrow().content(),
        vec![
            DialogContent::Choice {
                text: "With Friends !".to_string(),
                condition: Some(DialogCondition::new(
                    None,
                    Some(vec![WorldEvent::HasFriend.to_string()])
                ))
            },
            DialogContent::Choice {
                text: "Alone...".to_string(),
                condition: None
            }
        ]
    );
}

#[test]
fn test_init_tree_from_file_complex_kinship_1() {
    let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | None\n- Do you want to work with me ? | None\n\n## Hugo\n\n- Hey! How are you ?\n\n## Hugo\n\n- I'm sure you'll do just fine without me.\n",
            ), DialogCustomInfos::new(
    WorldEvent::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(),
    Some((KARMA_MIN, KARMA_MAX)),
    TriggerEvent::iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(),
));

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));
    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Hello".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "Do you want to work with me ?".to_string(),
                condition: None
            }
        ]
    );

    // println!("{}", root.borrow().print_file());

    // By choosing the n-eme choice, you will get the result of the n-eme child.

    assert_eq!(
        *root.borrow().children()[0].borrow().author(),
        Some(String::from("Hugo"))
    );
    assert_eq!(
        *root.borrow().children()[0].borrow().content(),
        vec![DialogContent::Text("Hey! How are you ?".to_string())]
    );

    assert_eq!(
        *root.borrow().children()[1].borrow().author(),
        Some(String::from("Hugo"))
    );
    assert_eq!(
        *root.borrow().children()[1].borrow().content(),
        vec![DialogContent::Text(
            "I'm sure you'll do just fine without me.".to_string()
        )]
    );
}

// TODO: add test for multiple throwable event `-> HasFriend, FightEvent\n`

#[test]
fn test_init_tree_from_file_throwable_event_1() {
    let root = init_tree_file(String::from(
                "# Morgan\n\n- Let's Talk | None\n- Let's Fight | None\n\n## Hugo\n\n- :)\n\n-> HasFriend\n\n## Hugo\n\n- :(\n\n-> FightEvent\n",
            ), DialogCustomInfos::new(
                WorldEvent::iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                Some((KARMA_MIN, KARMA_MAX)),
                TriggerEvent::iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));
    assert_eq!(
        *root.borrow().content(),
        vec![
            DialogContent::Choice {
                text: "Let's Talk".to_string(),
                condition: None
            },
            DialogContent::Choice {
                text: "Let's Fight".to_string(),
                condition: None
            }
        ]
    );

    // println!("{}", root.borrow().print_file());

    // By choosing the n-eme choice, you will get the result of the n-eme child.

    // first child
    assert_eq!(
        *root.borrow().children()[0].borrow().author(),
        Some(String::from("Hugo"))
    );
    assert_eq!(
        *root.borrow().children()[0].borrow().content(),
        vec![DialogContent::Text(":)".to_string())]
    );
    assert_eq!(
        *root.borrow().children()[0].borrow().trigger_event(),
        vec![TriggerEvent::HasFriend.to_string()]
    );

    // second child
    assert_eq!(
        *root.borrow().children()[1].borrow().author(),
        Some(String::from("Hugo"))
    );
    assert_eq!(
        *root.borrow().children()[1].borrow().content(),
        vec![DialogContent::Text(":(".to_string())]
    );
    assert_eq!(
        *root.borrow().children()[1].borrow().trigger_event(),
        vec![TriggerEvent::FightEvent.to_string()]
    );
}

#[test]
fn test_init_tree_from_file_except_1() {
    let root = init_tree_file(
        String::from(
            "# Morgan\n\n- Bonjour Florian. /\nComment vas/-tu :/# ? /\nJ'ai faim. /<3 /</|3\n",
        ),
        DialogCustomInfos::new(
            WorldEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            Some((KARMA_MIN, KARMA_MAX)),
            TriggerEvent::iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        ),
    );

    assert_eq!(*root.borrow().author(), Some(String::from("Morgan")));

    assert_eq!(
        *root.borrow().content(),
        vec![DialogContent::Text(
            "Bonjour Florian. \nComment vas-tu :# ? \nJ'ai faim. <3 <|3".to_string()
        )]
    );
}

// #[test]
// fn test_add_child() {
//     let root = init_tree_flat(String::from("[0,1,[3,4,5,[7,8]],2]"));
//     let new_node = Rc::new(RefCell::new(TreeNode::new()));
//     new_node.borrow_mut().value = Some(9);
//     let child = &*root.borrow().children()[2];
//     child.borrow_mut().add_child(new_node);
//     assert_eq!(root.borrow().print_flat(), "[0,1,[3,4,5,[7,8],9],2]");
// }
