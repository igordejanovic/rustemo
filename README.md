# Rustemo

LR/GLR parser generator for Rust (currently only LR). In the early phase of
the development.

## Why this name?

Rustemo is pronounced the same as Serbian word "растемо" which means "we grow".
The name is a tribute to the awesome and ever growing Rust community.

## Goals

- Excellent usability and error reporting. Rustemo should be easy to use. Each
  error should be caught and explained with sufficient details. The docs should
  always be up-to-date and all docs examples should be tested by CI.
- Clean separation of CFG grammar and semantic actions written in Rust so a
  regular editors can be used for editing Rust code to its full potential. At
  the same time the syntax of the language is kept clean and existing grammars
  can be easily ported to Rustemo.
- Syntax sugar for common patterns. E.g. zero-or-more(`*`), one-or-more (`+`),
  optional(`?`), groups (`()`), multiple match with a separator etc.
- A tool for automatic porting of grammars from other popular parsers (e.g.
  ANTLR).
- Clean separation between lexer, parser and builder. Parser asks lexer for next
  tokens during parsing and provides what is expected due to the current parsing
  context. This avoids certain classes of lexical ambiguities. Parser calls
  builder to produce the result on each parser operation.
- Lexer and builder can be provided by the user or auto-generated. When provided
  by user Rustemo can be used to parse virtually any kind of sequence and also
  build anything.
- Multiple builders can be called by providing a macro builder. E.g. you can
  construct an AST and a full/concrete syntax tree (for example, if you are
  implementing refactoring engine) where AST nodes keep relations to CST.
- There are a reasonable number of tests. I usually write tests before
  implementing each new feature (TDD). Tests are a good source of info until the
  docs are improved.

## Roadmap

- [x] LR parsing.
- [x] Bootstrapping. Rustemo is implemented in itself.
- [x] Actions providing AST build are auto-generated but can be manually
      modified. Manual modifications are preserved on code re-generation while
      new types/actions are added to the file. This allow for fast development
      while keeping full control over the AST.
- [x] Regex-like syntax sugar
- [ ] Parenthesized groups. Still not sure if this is a good thing to have.
      Sometimes it can nicely reduce clutter but if used too much it lowers
      readability.
- [x] Disambiguation filters: priorities, associativities.
- [x] Rule/production meta-data. E.g. production kinds.
- [x] CLI and API
- [x] Tracking of position and reporting error with line/column works.
- [ ] Support for a layout (comments, whitespaces given as CFG). It'll be
      implemented as a special grammar rule and parsed by the LR parser. Result
      will be passed by context to actions which can do whatever they please.
      E.g. a generic tree builder can keep layout on the following tree leaf.
- [ ] Better error reporting during grammar analysis and state machine building.   
      Basic error reporting is implemented at the moment without full location
      info.
- [ ] Macro builder.
- [ ] Docs (will mostly be based on parglare docs for the grammar language).
- [ ] GLR parsing based on Right-Nulled GLR algorithm (RNGLR).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Bootstrapping approach and the pieces of code are based on the [LALRPOP
project](https://github.com/lalrpop/lalrpop).

## Similar projects

The architecture and the general idea of Rustemo is loosely based on a similar
project for Python, called
[parglare](https://github.com/igordejanovic/parglare), I've started several
years ago.

I have found a lot of inspiration and ideas in the following projects:

- [LALRPOP](https://github.com/lalrpop/lalrpop) - LR(1) parser generator for
  Rust. This project is the most similar to Rustemo so I've found there a lot of
  nice ideas.
- [Nom](https://github.com/Geal/nom) - Parser combinator library. Nice
  architecture and nicely designed traits.
- [pest](https://github.com/pest-parser/pest) - PEG parser for Rust. Seems nice
  and well maintained.
