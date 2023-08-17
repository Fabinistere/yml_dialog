# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## API Guideline - [v0.2.1](https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.1) - 2023-08-18

[![v0.2.1](https://img.shields.io/badge/v0.2.1-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.1)](https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.1)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/yml_dialog/commits/v0.2.1)](https://github.com/fabinistere/yml_dialog/commits/v0.2.1)

### Changed

- `Cargo.toml` includes all common metadata ([C-METADATA](https://rust-lang.github.io/api-guidelines/documentation.html#c-metadata))
  - authors, description, license, homepage, documentation, repository, keywords, categories
- Types eagerly implement common traits ([C-COMMON-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits))
- rename `DialogNodeYML` to `DialogNode`

## YML Dialog - [v0.2.0](https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.0) - 2023-08-17

[![v0.2.0](https://img.shields.io/badge/v0.2.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.0)](https://github.com/Fabinistere/yml_dialog/releases/tag/v0.2.0)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/yml_dialog/commits/v0.2.0)](https://github.com/fabinistere/yml_dialog/commits/v0.2.0)

### Preview

[![complete_example](https://github.com/Fabinistere/yml_dialog/assets/73140258/731025d7-9eed-4b92-a820-a175bf886df7)](https://fabinistere.github.io/yml_dialog/)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/yml_dialog#license)
- [Web Demo](https://fabinistere.github.io/yml_dialog/)
- Total Migration to YML format

### Removed

- Markdown interpretor and Tree Structure (treenode)
