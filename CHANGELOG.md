# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

# Added

- Support for `Forest` iteration (`into_iter()`, and `iter()`).

# Fixed

- Getting `CARGO_WORKSPACE_DIR` in `local_file!`.

# Changed

- Removed lexer type parameter from the `Parser` trait.


## [0.2.0] - 2023-10-22

# Added

- GLR parsing based on Right-Nulled GLR algorithm (RNGLR).
  - Base Tomita's algorithm. Shared packed parse forest.
  - Lazy tree extraction from forest.
  - Calling arbitrary builder over extracted tree.
  - Support for EMPTY productions through RN table entries (RNGLR algorithm).


## [0.1.0] - 2023-06-02

- Initial release. See the README for the features available in this release.


[unreleased]: https://github.com/igordejanovic/rustemo/compare/0.2.0...HEAD
[0.2.0]: https://github.com/igordejanovic/rustemo/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/igordejanovic/rustemo/releases/tag/0.1.0
