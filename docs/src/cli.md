# Rustemo CLI

Crate `rustemo-compiler` installs a binary `rcomp` which is a CLI to the Rustemo
compiler.

```admonish note
Instead of calling CLI manually you can setup your project to call the Rustemo
compiler from `build.rs` script. You can read more in the [configuration
section](configuration.md).
```

To get all the option of the `rcomp` you can run `rcomp --help`.

The only mandatory argument is the path to `.rustemo` file from which you want
the parser to be generated.

```
rcomp my_grammar.rustemo
```

The default lexer is a string lexer which uses [string/regex
recognizers](grammar_language.md#terminals) from the grammar.

The default builder will call auto-generated actions and [create automatically
deduced AST](builders.md#default-builder).
