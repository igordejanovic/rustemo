# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

## Fixed

- call lexer after reduction in LR parser as the set of possible tokens may change.


# [0.5.0] - 2024-01-23

## Added

- `dot` export marks conflict states in red color.
- More special grammars tests.

## Fixed
- Pager's compatibility test for LALR states merging/splitting. Implemented
  Menhir's version.
  
## Changed
- Table type in CLI renamed from `lalr-pagerw` to `lalr-pager`.


# [0.4.0] - 2024-01-14

## Added

- Support for lexical disambiguation strategies. See [the docs](https://www.igordejanovic.net/rustemo/lexers.html#lexical-disambiguation).

## Fixed

- Bug in GLR in the context of lexical ambiguity
- Bug in `string_difference` used in tests.
- Regex recognizers anchoring which lead to unanchored search when `|` was used.

## Changed

- Minimal supported Rust version is 1.74.
- Default parser tables implementation is function-based.


# [0.3.0] - 2023-12-27

## Added

- Support for `Forest` iteration (`into_iter()`, and `iter()`).
- Multiple parser table styles generation: array-based, function-based.
  Different tradeoffs. Can be configured using settings API or `rcomp`
  parameter.

## Fixed

- Getting `CARGO_WORKSPACE_DIR` in `local_file!`.
- Correctly report error in GLR parsing.
- Handle >2 conflicts per state.
- `build.rs` to prevent unnecessary rebuilds.

## Changed

- Removed lexer type parameter from the `Parser` trait.
- Removed `Rc` from lexer API.
- `ParserDefinition` trait change.


# [0.2.0] - 2023-10-22

## Added

- GLR parsing based on Right-Nulled GLR algorithm (RNGLR).
  - Base Tomita's algorithm. Shared packed parse forest.
  - Lazy tree extraction from forest.
  - Calling arbitrary builder over extracted tree.
  - Support for EMPTY productions through RN table entries (RNGLR algorithm).


# [0.1.0] - 2023-06-02

- Initial release. See the README for the features available in this release.


[unreleased]: https://github.com/igordejanovic/rustemo/compare/0.5.0...HEAD
[0.5.0]: https://github.com/igordejanovic/rustemo/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/igordejanovic/rustemo/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/igordejanovic/rustemo/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/igordejanovic/rustemo/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/igordejanovic/rustemo/releases/tag/0.1.0
