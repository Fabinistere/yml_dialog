use fto_dialog::*;
// use serde::Serialize;
#[cfg(test)]
use std::collections::BTreeMap;

#[test]
fn test_yaml_monolog_serialize_1() {
    let mut map = BTreeMap::new();
    map.insert(
        1,
        DialogNodeYML::new(
            "Le Pape".to_string(),
            Content::Monolog {
                text: vec![String::from("Hello Homie")],
                exit_state: 2,
            },
            vec![],
        ),
    );

    let yaml = serde_yaml::to_string(&map).unwrap();
    // println!("{}", yaml);

    assert_eq!(
        yaml,
        "1:
  source: Le Pape
  content:
    text:
    - Hello Homie
    exit_state: 2
  trigger_event: []\n"
            .to_string()
    )
}

#[test]
fn test_yaml_monolog_serialize_2() {
    let mut map = BTreeMap::new();
    map.insert(
        1,
        DialogNodeYML::new(
            "The Frog".to_string(),
            Content::Monolog {
                text: vec![
                    String::from("Hello Homie"),
                    String::from("I mean..."),
                    String::from("KeroKero"),
                ],
                exit_state: 2,
            },
            vec![],
        ),
    );
    map.insert(
        2,
        DialogNodeYML::new(
            "Random Frog".to_string(),
            Content::Monolog {
                text: vec![String::from("KeroKero")],
                exit_state: 3,
            },
            vec![],
        ),
    );

    let yaml = serde_yaml::to_string(&map).unwrap();
    // println!("{}", yaml);

    assert_eq!(
        yaml,
        "1:
  source: The Frog
  content:
    text:
    - Hello Homie
    - I mean...
    - KeroKero
    exit_state: 2
  trigger_event: []
2:
  source: Random Frog
  content:
    text:
    - KeroKero
    exit_state: 3
  trigger_event: []\n"
            .to_string()
    )
}

#[test]
fn test_yaml_choices_serialize_1() {
    let mut map = BTreeMap::new();
    map.insert(
        1,
        DialogNodeYML::new(
            "The Frog".to_string(),
            Content::Choices(vec![
                Choice::new(String::from("Hello HomeGirl"), None, 2),
                Choice::new(String::from("KeroKero"), None, 3),
            ]),
            vec![],
        ),
    );
    map.insert(
        2,
        DialogNodeYML::new(
            "Random Frog".to_string(),
            Content::Monolog {
                text: vec![String::from("Yo Homie")],
                exit_state: 4,
            },
            vec![],
        ),
    );
    map.insert(
        3,
        DialogNodeYML::new(
            "Random Frog".to_string(),
            Content::Monolog {
                text: vec![String::from("KeroKero")],
                exit_state: 4,
            },
            vec![],
        ),
    );

    let yaml = serde_yaml::to_string(&map).unwrap();
    // println!("{}", yaml);

    assert_eq!(
        yaml,
        "1:
  source: The Frog
  content:
  - text: Hello HomeGirl
    condition: null
    exit_state: 2
  - text: KeroKero
    condition: null
    exit_state: 3
  trigger_event: []
2:
  source: Random Frog
  content:
    text:
    - Yo Homie
    exit_state: 4
  trigger_event: []
3:
  source: Random Frog
  content:
    text:
    - KeroKero
    exit_state: 4
  trigger_event: []\n"
            .to_string()
    )
}

#[test]
fn test_yaml_monolog_deserialize_field_missing() {
    let yaml = "1:
  source: Le Pape
  content:
    text:
    - Hello Homie
    exit_state: 2\n";

    let mut map = BTreeMap::new();
    map.insert(
        1,
        DialogNodeYML::new(
            "Le Pape".to_string(),
            Content::Monolog {
                text: vec![String::from("Hello Homie")],
                exit_state: 2,
            },
            vec![],
        ),
    );

    let deserialized_map: BTreeMap<usize, DialogNodeYML> = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(map, deserialized_map)
}

// #[derive(Serialize, Deserialize, Default)]
// struct Condition(Vec<WorldEvent>);

// #[derive(Serialize, Deserialize, Default)]
// #[serde(untagged)]
// enum WorldEvent {
//     #[default]
//     HasCharisma,
//     HasFriends,
// }

// #[test]
// fn test_generic_type_deserialize() {
//     let yaml = "1:
//   source: La Pape
//   content:
//     text:
//     - Hello Homie
//     exit_state: 2
// 2:
//   source: The Frog
//   content:
//   - text: Hello HomeGirl
//     condition: !Condition
//     - HasCharisma
//     exit_state: 3
//   - text: KeroKero
//     condition: null
//     exit_state: 1
//   trigger_event: []\n";

//     let mut map = BTreeMap::new();
//     map.insert(
//         1,
//         DialogNodeYML::new(
//             "Le Pape".to_string(),
//             Content::Monolog {
//                 text: vec![String::from("Hello Homie")],
//                 exit_state: 2,
//             },
//             vec![],
//         ),
//     );
//     map.insert(
//         2,
//         DialogNodeYML::new(
//             "The Frog".to_string(),
//             Content::Choices(vec![
//                 Choice::new(
//                     "Hello HomeGirl".to_string(),
//                     Some(Condition(vec![WorldEvent::HasCharisma])),
//                     3,
//                 ),
//                 Choice::new("KeroKero".to_string(), None, 1),
//             ]),
//             vec![],
//         ),
//     );

//     let deserialized_map: BTreeMap<usize, DialogNodeYML> = serde_yaml::from_str(&yaml).unwrap();

//     assert_eq!(map, deserialized_map)
// }
