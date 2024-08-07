![](https://raw.githubusercontent.com/igordejanovic/rustemo/main/art/rustemo-logo-small.png)

# Introduction

Rustemo is a LR/GLR parser generator for Rust (a.k.a.
[compiler-compiler](https://en.wikipedia.org/wiki/Compiler-compiler)).

Basically, this kind of tools, given a formal grammar of the language, produce a
program that can transform unstructured text (a sequence of characters, or more
generally a sequence of tokens) to a structured (tree-like or graph-like) form
which is more amenable to further programmatical analysis.

One interesting feature of Rustemo is Abstract-Syntax Tree[^ast] (AST)
auto-generation, i.e. based on a formal grammar of the language Rustemo will
deduce Rust types and actions used to create and represent AST of your language.
These types and actions can further be incrementally manually tuned to your
likings. You can find more info in the section on the [default
builder](builders.md#default-builder).

Rustemo tries to provide sensible defaults but is made with a flexibility in
mind. Thus, you can plug-in your own builder or/and lexer.

See [the project
README](https://github.com/igordejanovic/rustemo/blob/main/README.md) for
features, aspirations and the road-map.

There are multiple alternatives to this project. Some of them are listed in the
[Similar projects](https://github.com/igordejanovic/rustemo/#similar-projects)
section in the README. I advise you to check them also to be able to make an
informed decision of what approach would suit you the best.

# Installation and setup

Rustemo uses `cargo` for the project management. There are two crates of
interest:
- `rustemo-compiler` - is a crate you will use during the development. This
  crate provides the compiler [CLI `rcomp`](./cli.md). You will also used this
  crate as a build dependency if the compiler is called from the `build.rs`
  script. See the [configuration chapter](./configuration.md) for more
  information.
- `rustemo` - is Rustemo runtime. This crate will be used by the generated
  parser. If you use the default string lexer you will need additional
  dependencies. See the [calculator tutorial](./tutorials/calculator/calculator.md).

# A usual workflow

To work with Rustemo a usual sequence of steps is as follows (after installing
`rustemo-compiler` crate):
1. Write a grammar in a textual file with `.rustemo` extension. For example, a
   JSON grammar might look like this (see the examples directory):

```
{{#include ../../examples/json/src/json.rustemo}} 
```

2. Run `rcomp` compiler (a binary installed from `rustemo-compiler` crate) with
   the given grammar to produce the parser code and optional builder actions
   (enabled by default).
3. Fix errors reported by `rcomp` if any and repeat from step 2 until there is
   no errors. See the [chapter on handling
   errors](./handling_errors/handling_errors.md).
4. Call `parse` method from the generated parser with the input you want to
   parse (see [the calculator tutorial](./tutorials/calculator/calculator.md)
   for the details).
   
Instead of calling `rcomp` you can setup `build.rs` script to generate the
parser whenever you build your crate. You can find detailed instruction on how
to call the parser from the `build.rs` script and use the generated modules in
the [configuration section](configuration.md).

# Where to start?

The best place to start at the moment is [the calculator
tutorial](./tutorials/calculator/calculator.md) with a reference to the [grammar
language](grammar_language.md) section and other sections as needed.

User [fogarecious](https://github.com/fogarecious) has contributed a [simple
tutorial](https://github.com/fogarecious/rustemo_tutorial) geared towards
Rustemo beginners so, though unofficial, this is also a good material to read
while learning Rustemo.

Besides the tutorial and this docs, another source of information are
[integration tests](https://github.com/igordejanovic/rustemo/tree/main/tests).
Tests are usually implemented by a grammar where Rustemo compiler is called from
the
[build.rs](https://github.com/igordejanovic/rustemo/blob/main/tests/build.rs)
script. The result of each test is persisted to a `.ast` (or `.err` if error is
expected) file if it doesn't exist, or compared with the expected file if the
file exists. Be sure to check these tests as they provide a good overview of the
Rustemo possibilities.


# Footnotes

[^ast]: See the note in [the section on default builders](builders.md#default-builder).
