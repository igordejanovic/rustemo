# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

## Added

- `builder-loc-info` config option which provides location information in the
  default builder generated AST types. See the discussion at [2] and [the
  docs](https://www.igordejanovic.net/rustemo/builders.html)
  
[2]: https://github.com/igordejanovic/rustemo/issues/2


# [0.6.3] - 2024-11-11

## Fixed

- attempt to subtract causes overflow in GLR default builder. See [16]. Thanks
  andrewbaxter@GitHub for the bug report and regression test.
- reexported dependencies for generated parsers. Now, the only dependency
  required by the client code is `rustemo`. See [15]. Thanks andrewbaxter@GitHub
  for the idea
  
[16]: https://github.com/igordejanovic/rustemo/issues/16
[15]: https://github.com/igordejanovic/rustemo/issues/15


# [0.6.2] - 2024-10-11

## Fixed

- `unwrap` panic if rustemo-compiler is built outside of git repo (e.g. from crates.io).


# [0.6.1] - 2024-10-02

## Changed

- Development setup changed to use [Nix package manager](https://nixos.org/).
  See [12] and [Contributing
  guide](https://www.igordejanovic.net/rustemo/contributing.html). Thanks
  AlexSherbinin@GitHub for the contribution.

## Fixed

- Always regenerate `_actions.rs` in `OUT_DIR` folder. See [13]. Thanks
  safinaskar@GitHub for reporting.
- Override coloring during dot export.

[12]: https://github.com/igordejanovic/rustemo/pull/12
[13]: https://github.com/igordejanovic/rustemo/issues/13


# [0.6.0] - 2024-02-20

## Added

- disable trace log output with switch `--notrace` or env variable
  `RUSTEMO_NOTRACE`. Trace log is sent to std error. Thanks
  @stevefan1999-personal for reporting. See [4].
- [fancy-regex] support. Thanks @stevefan1999-personal. See [8].

## Fixed

- call lexer after reduction in LR parser as the set of possible tokens may change.
- string escaping in Rustemo grammars

## Changed

- improved debug trace prints.
- improved LR automata table print output.

[4]: https://github.com/igordejanovic/rustemo/issues/4
[8]: https://github.com/igordejanovic/rustemo/pull/8
[fancy-regex]: https://github.com/fancy-regex/fancy-regex


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


[unreleased]: https://github.com/igordejanovic/rustemo/compare/0.6.3...HEAD
[0.6.3]: https://github.com/igordejanovic/rustemo/compare/0.6.2...0.6.3
[0.6.2]: https://github.com/igordejanovic/rustemo/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/igordejanovic/rustemo/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/igordejanovic/rustemo/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/igordejanovic/rustemo/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/igordejanovic/rustemo/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/igordejanovic/rustemo/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/igordejanovic/rustemo/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/igordejanovic/rustemo/releases/tag/0.1.0
