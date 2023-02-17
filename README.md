# Rustemo

LR/GLR parser generator for Rust (currently only LR). 

**In the early phase of development. DO NOT USE IN PRODUCTION!**

## Aspirations

- **Both LR and GLR parsing from the same grammar**
  
  E.g. start with GLR for easier development and refactor to LR for performance,
  or start from LR and move to GLR if your language needs more than 1 token of
  lookahead or is inherently ambiguous.

- **Usability and error reporting**

  Rustemo should be easy to use with sane defaults. Each error should be caught
  and explained with sufficient details. The docs should always be up-to-date
  and all docs examples should be tested by CI.

- **Clean separation of CFG grammar and semantic actions written in Rust**

  So a regular editors can be used for editing Rust code to its full potential.
  At the same time the syntax of the language is kept clean and existing
  grammars can be easily ported to Rustemo.

- **Syntax sugar for common patterns**

  E.g. zero-or-more(`*`), one-or-more (`+`), optional(`?`), groups (`()`),
  multiple match with a separator etc.

- **Clean separation between lexer, parser and builder**

  A parser asks a lexer for next tokens during parsing while telling the lexer
  what is expected due to the current parsing context. This avoids certain
  classes of lexical ambiguities. The parser calls builder to produce the result
  on each parser operation.

- **Flexibility**

  Default lexers and builders are provided/generated out-of-the box but the user
  can choose to write custom lexer and/or builder.
  
  When custom lexer/builder is provided Rustemo can be used to parse virtually
  any kind of sequence and also build any kind of output.

- **Inference of AST node types from the grammar**
  
  For default built-in builder, AST node types and semantics actions should be
  inferred from the grammar and auto-generated, but the user can introduce
  manual changes.

- **Multiple builders can be called by providing a macro builder**

  E.g. you can construct an AST and a full/concrete syntax tree (for example, if
  you are implementing refactoring engine) where AST nodes keep relations to
  CST.

- **High test coverage**

  There are a reasonable number of tests. I usually write tests before
  implementing each new feature (TDD). Tests are a good source of info until the
  docs are improved.


## Roadmap (tentative)

### v0.1.0
- [x] LR parsing.
- [x] Bootstrapping. Rustemo is [implemented in itself](./rustemo/src/lang/).
- [x] Actions providing AST build are auto-generated but can be manually
      modified. Manual modifications are preserved on code re-generation while
      new types/actions are added to the file. This allow for fast development
      while keeping full control over the AST.
- [x] Regex-like syntax sugar. See [the tests](./tests/src/sugar/).
- [ ] Parenthesized groups. Still not sure if this is a good thing to have.
      Sometimes it can nicely reduce clutter but if used too much it lowers
      readability.
- [x] Repetition modifiers (e.g. separator)
- [x] Disambiguation filters: priorities, associativities.
- [x] Rule/production meta-data. E.g. production kinds.
- [x] CLI and API. A `rustemo` is available that can be called on Rustemo
      grammars. Also an API enables integrating parser compiling into Rust
      `build.rs` scripts. See [the calculator example](./examples/calculator/)
      or [integration tests](./tests/).
- [x] Tracking of position and reporting error with line/column works.
- [x] Support for a layout (comments, whitespaces given as CFG). It is
      implemented as a special grammar rule and parsed by the LR parser. Result
      is be passed by context to actions which can do whatever they please. E.g.
      a generic tree builder keep layout on the following tree leaf.
- [ ] Better error reporting during grammar analysis and state machine building.   
      Basic error reporting is implemented at the moment without full location
      info.
- [ ] Docs (will mostly be based on parglare docs for the grammar language).
      There are some WIP in the docs folder.

### v0.2.0
- [ ] Macro builder.
- [ ] Greedy repetitions.
- [ ] GLR parsing based on Right-Nulled GLR algorithm (RNGLR).

### v1.0
- [ ] Grammars compositions. Grammars importing, rule inheritance etc.
- [ ] Better disambiguations (investigate dynamic).
- [ ] Visualizations/debugging of GLR parsing process.

### Post v1.0
- [ ] Investigate possibility to implement Elkhound style of LR/GLR switching
- [ ] Investigate possibility to implement incremental parsing
- [ ] Tooling for working with Rustemo grammars (e.g. LSP server, plugins for
      popular editors)


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


## Why this name?

Rustemo is pronounced the same as Serbian word "растемо" which means "we grow".
The name is a tribute to the awesome and ever growing Rust community.
