# YML based Dialog Structure

<!-- [![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking) -->
<!-- [![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-0.10-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking) -->
[![v0.2.2](https://img.shields.io/badge/v0.2.2-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.2.2)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.2.2) [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_turn-based_combat#license)

<!--
| bevy | yml_dialog |
|------|------------|
| 0.10 | 0.2.1      |
| 0.10 | 0.2.0      |
-->

## [Example](https://fabinistere.github.io/yml_dialog/)

[![complete_example](https://github.com/Fabinistere/yml_dialog/assets/73140258/9f6a57d4-c0d8-43a0-aa56-a0873c7c0728)](https://fabinistere.github.io/yml_dialog/)

If you want to compile the code at home, [download assets here](https://cloud.disroot.org/s/sSsjHxXpTH88oyW): disroot's cloud,
clone the repo, extract the assets in the root of the repo and run `cargo run --example complete_example`.

## RoadMap

- [ ] fully functional API to implement a dialog system into a (bevy game or any) app
  - [ ] Usage
    - [ ] [Custom (De)Serialize implementation](https://serde.rs/impl-serialize.html).
      The field `content:` is interpreted as either `monolog:` or `choices:`
    - [ ] [Generic type](https://doc.rust-lang.org/reference/items/generics.html) for `Condition`
    - [ ] [Generic type](https://doc.rust-lang.org/reference/items/generics.html) for an `extra` field on the DialogNode
    - [ ] [Dynamic Macros](https://stackoverflow.com/a/63849405)
  - [x] Exclude Bevy dependency
  - [ ] Follow all guidelines from [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
    - [ ] **Documentation**
      - [ ] Examples use `?`, not `try!`, not `unwrap` ([C-QUESTION-MARK](https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark))
      - [x] Crate level docs are thorough and include examples ([C-CRATE-DOC](https://rust-lang.github.io/api-guidelines/documentation.html#c-crate-doc))
      - [x] create an example
    - [x] **Interoperability**
      - [x] Types eagerly implement common traits
      - [x] ...
    - [ ] **Future proofing** (crate is free to improve without breaking users' code)
      - [ ] Sealed traits protect against downstream implementations ([C-SEALED](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed))
      - [x] Structs have private fields ([C-STRUCT-PRIVATE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-private))
      - [ ] Newtypes encapsulate implementation details ([C-NEWTYPE-HIDE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-newtype-hide))
      - [ ] Data structures do not duplicate derived trait bounds ([C-STRUCT-BOUNDS](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-bounds))

## Contribute

Release's format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This project also follows to [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/).

We have to respect all the [rust API convention](https://rust-lang.github.io/api-guidelines/checklist.html).
For the lore, follow all [Bevy plugins guidelines](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md).

Run `cargo clippy`, `cargo doc` before commit.

## License

This project is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

See the very good reasons for including both [here](https://github.com/bevyengine/bevy/issues/2373).

## Inspiration

- [Medium: A Tree Structure implemented in Rust.](https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75)
- [Wikipedia: Dialogue Tree](https://en.wikipedia.org/wiki/Dialogue_tree#)
- [Wikipedia: Nonlinear Gameplay](https://en.wikipedia.org/wiki/Nonlinear_gameplay)
- [Bevy's SubReddit: Bevy Dialog Discussion](https://www.reddit.com/r/bevy/comments/wr22n5/ideas_on_the_basic_interface_for_a_dialogue_system/)

I began coding this as a tree strucutre dialog but a graph based is much more suited for a dialog. Maybe after this migration this crate is not needed anymore and will just do a devlog. (and free this name :)

## Naming

Accordingly to this [discussion](https://github.com/bevyengine/bevy/discussions/1202),
this is not a trivial question.

- ***yml-dialog***
- dialog-structure
- md-to-dialog-tree
- fto-dialog
- bevy_dialog
- bevy_dialog_system
- bevy_fto_dialog / bevy_fob_dialog
