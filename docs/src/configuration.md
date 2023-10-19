# Configuration

The parser generator configuration is exposed through API calls which can be
imported from the root of the `rustemo_compiler` crate:

- `pub fn process_crate_dir()` - recursivelly visit all the grammars starting
  from the crate root directory and generate the parsers and actions.
- `pub fn process_dir<P: AsRef<Path>>(dir: P)` - recursivelly visit all the
  grammars starting from the given directory and generate the parsers and
  actions.
- `pub fn process_grammar<P: AsRef<Path>>(grammar: P)` - generate the parser and
  actions for the given grammar.
- `Settings::new()` - returns a default `Settings` which can be further
  configured using chained calls.

These are meant to be called from the `build.rs` script. The most basic usage
would be:

```rust
{{#include ../../examples/calculator/build.rs }}
```

```admonish note
Don't forget to add `rustеmo-compiler` to the `build-dependencies` section of the
`Cargo.toml` file.
```

In this example we are using the default settings and run the recursive
processing of the project dir as determined by the cargo `CARGO_MANIFEST_DIR`
environment variable. By default, it will generate both parser and actions in
the cargo output dir as determined by `OUT_DIR` env variable.

You can change the default output to be the source tree, i.e. to generate
parser/actions next to the grammar file by calling:

```rust
rustemo_compiler::Settings::new().in_source_tree().process_dir()
```

This will create default settings, change settings to generate the parser in the
source tree and then process the current project directory.

It is usually discouraged to modify the source tree from the `build.rs` script
but in the case of actions it is usually necessary as the actions are generated
and manually maintained. To generate just the actions in the source tree while
keeping the parser in the output directory you can do the following:

```rust
rustemo_compiler::Settings::new().actions_in_source_tree().process_dir()
```

```admonish note
When running rustemo from `build.rs` your crate must have a build dependency to
`rustemo-compiler`. If you don't want this than you can always resort to
building your parser using [rustemo CLI](./cli.md). Just don't forget to
manually regenerate the parser and commit changes to your version control system
when you change the grammar.
```

```admonish todo
For the full docs on the settings provided see the [crate docs](). Provide the link...
```

## Using generated modules

When the parser/actions is generated in the source tree you use it as any other
Rust module but when any of them is generated in the output you can't include
them in the module tree as they are generated in the output dir.

To be able to include modules from the output dirs you use `rustemo_mod!` macro:

```rust
use rustemo::rustemo_mod;
rustemo_mod! {pub(crate) rustemo, "/src/lang"}
```

or

```rust
use rustemo::rustemo_mod;
rustemo_mod!(generic_tree, "/src/builder/generic_tree");
```

This macro accepts two parameters. The first is a usual syntax used with `mod`
(attributes, visibility and the name of the module) while the second parameter
is the path to the module/grammar directory from the project root. This second
parameter is needed for the macro to be able to calculate the full path in the
output directory.

