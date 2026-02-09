![](https://raw.githubusercontent.com/igordejanovic/rustemo/main/art/rustemo-logo-small.png)

# Introduction

Rustemo is an LR/GLR parser generator for Rust (a.k.a.
[compiler-compiler](https://en.wikipedia.org/wiki/Compiler-compiler)).

Basically, these kinds of tools, given a formal grammar of the language, produce
a program that can transform unstructured text (a sequence of characters, or
more generally a sequence of tokens) into a structured (tree-like or graph-like)
form which is more amenable to further programmatic analysis.

One interesting feature of Rustemo is Abstract-Syntax Tree[^ast] (AST)
auto-generation, i.e., based on a formal grammar of the language, Rustemo will
deduce Rust types and actions used to create and represent the AST of your
language. These types and actions can further be incrementally manually tuned to
your liking. You can find more info in the section on the [default
builder](builders.md#default-builder).

Rustemo tries to provide sensible defaults but is made with flexibility in mind.
Thus, you can plug in your own builder and/or lexer.

See [the project
README](https://github.com/igordejanovic/rustemo/blob/main/README.md) for
features, aspirations, and the road map.

There are multiple alternatives to this project. Some of them are listed in the
[Similar projects](https://github.com/igordejanovic/rustemo/#similar-projects)
section in the README. I advise you to check them also to be able to make an
informed decision on what approach would suit you best.

# Installation and setup

Rustemo uses `cargo` for project management. There are two crates of interest:

- `rustemo-compiler`: This is a crate you will use during development. It
  provides the compiler CLI, `rcomp` (see [CLI documentation](./cli.md)). You
  will also use this crate as a build dependency if the compiler is called from
  the `build.rs` script. See the [configuration chapter](./configuration.md) for
  more information.

- `rustemo`: This is the Rustemo runtime. This crate will be used by the
  generated parser. If you use the default string lexer, you will need
  additional dependencies. See the [calculator
  tutorial](./tutorials/calculator/calculator.md).

# A usual workflow

To work with Rustemo, a usual sequence of steps is as follows (after installing
the `rustemo-compiler` crate):

1. Write a grammar in a textual file with a `.rustemo` extension. For example, a
   JSON grammar might look like this (see the examples directory):

```
{{#include ../../examples/json/src/json.rustemo}}
```

2. Run the `rcomp` compiler (a binary installed from the `rustemo-compiler`
   crate) with the given grammar to produce the parser code and optional builder
   actions (enabled by default).
   
3. Fix errors reported by `rcomp` if any and repeat from step 2 until there are
   no errors. See the [chapter on handling
   errors](./handling_errors/handling_errors.md).

4. Call the `parse` method from the generated parser with the input you want to
   parse (see [the calculator tutorial](./tutorials/calculator/calculator.md)
   for details).
   
Instead of calling `rcomp`, you can set up a `build.rs` script to generate the
parser whenever you build your crate. You can find detailed instructions on how
to call the parser from the `build.rs` script and use the generated modules in
the [configuration section](configuration.md).

# Where to start?

Start with the [calculator tutorial](./tutorials/calculator/calculator.md).
Refer to the [grammar language](grammar_language.md) section and other sections
as needed. 

User [fogarecious](https://github.com/fogarecious) contributed a [simple
tutorial](https://github.com/fogarecious/rustemo_tutorial) for Rustemo
beginners. Though unofficial, it is a useful resource for learning Rustemo.

In addition to the tutorial and documentation, another information source is the
[integration tests](https://github.com/igordejanovic/rustemo/tree/main/tests).
Each test typically uses a grammar where the Rustemo compiler is invoked from
the [build.rs
script](https://github.com/igordejanovic/rustemo/blob/main/tests/build.rs). If
the resulting file does not exist, it is saved as a `.ast` or `.err` file (if an
error is expected). If the file exists, it is compared with the expected file.
Check these tests for insights on Rustemo's capabilities.

# Footnotes

[^ast]: See the note in [the section on default builders](builders.md#default-builder).
