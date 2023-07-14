# *Fabien et la Trahison de Olf*'s Dialog System

<!-- [![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking) -->
<!-- [![v0.1.0](https://img.shields.io/badge/v0.1.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.1.0)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.1.0) -->
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-0.10-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_turn-based_combat#license)

| bevy | fto-dialog |
|------|------------|
| 0.10 | 0.1.0      |

## Naming

Accordingly to this [discussion](https://github.com/bevyengine/bevy/discussions/1202),
this is not a trivial question.

- fto-dialog
- bevy_dialog
- bevy_dialog_system
- bevy_fto_dialog / bevy_fob_dialog

## Example

```rust
#[derive(Component)]
struct DialogNode
```

## Contribute

Release's format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This project also follows to [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/).

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
