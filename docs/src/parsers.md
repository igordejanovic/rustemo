# Parsers

Parsers use tokens from lexer as inputs and recognize syntactic elements. Then, they call a builder to produce the final output.

There are two flavours of parsers supported by Rustemo:

- Deterministic LR 
- Non-deterministic GLR, or more precise Right-Nulled GLR

```admonish tip
GLR parsing is more complex as it must handle all possibilities so there is some
overhead and LR parsing is generally faster. Thus, use GLR only if you know that
you need it or in the early development process when you want to deal with
SHIFT/REDUCE conflicts later.

Another benefit of LR parsing is that it is deterministic and non-ambiguous. If
the input can be parsed there is only one possible way to do it with LR.
```

The API for both flavours is similar. You create an instance of the generated
parser type and call either `parse` or `parse_file` where the first method
accepts the input directly while the second method accepts the path to the file
that needs to be parsed.

For example, in the calculator tutorial, we create a new parser instance and
call `parse` to parse the input supplied by the user on the stdin:

```rust
{{#include ./tutorials/calculator/calculator1/src/main.rs:main}}
```

The parser type `CalculatorParser` is generated by Rustemo from grammar
`calculator.rustemo`.

The result of the parsing process is a `Result` value which contains either the
result of parsing if successful, in the `Ok` variant, or the error value in
`Err` variant.

If deterministic parsing is used the result will be the final output constructed
by the [configured builder](./builders.md). 

For GLR the result will be `Forest` which contains all the possible
trees/solution for the given input. For the final output you have to choose the
tree and call the builder over it.

To generate GLR parser either set the algorithm using settings API (e.g. from `build.rs` script):

```rust
rustemo_compiler::Settings::new().parser_algo(ParserAlgo::GLR).process_dir()
```

or call `rcomp` CLI with `--parser-algo glr` over your grammar file.

For example of calling GLR parser see this test:

```rust
{{#include ../../tests/src/glr/forest/mod.rs:forest}}
```

The most useful API calls for `Forest` are `get_tree` and `get_first_tree`.
There is also `solutions` which gives your the number of trees in the forest.

`Forest` supports `into_iter()` and `iter()` so it can be used in the context of
a for loop.

```rust
{{#include ../../tests/src/glr/forest/mod.rs:forest-iter}}
```

A tree can accept a builder using the `build` method. For an example of calling
the default builder over the forest tree see this test:

```rust
{{#include ../../tests/src/glr/build/mod.rs:build}}
```
