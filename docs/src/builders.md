# Builders

A builder is a component that is called by the parser during the parsing process
to constructs the output.

Currently Rustemo can be configured with three builder types:

- **The default builder**

  When default builder is used, Rustemo will perform type inference for AST node
  types based on the grammar. The builder, AST types and actions for creating
  instances of AST nodes will be generated.
  
- **Generic tree builder**

  This builder builds a tree where each node is of `TreeNode` type.

- **Custom builder**

  Is provided by the user.


## Default builder

For default builder Rustemo will generate default AST types and actions
following a certain set of rules explained in this section. The generated
builder will then call into actions to produce instances of the AST types. The
final output of this builder is AST tailored for the given grammar.

The builder will be generated inside `<lang>.rs` file where `<lang>` is the name
of the grammar. The actions will be generated into `<lang>_actions.rs` file.

```admonish note
There are two approaches where generated files are stored. See... 
```

```admonish todo
Provide a link to the part of the docs where generation approaches are described.
```

### AST type inference

Based on grammar rules Rustemo will infer and generate AST node types which you
can modify afterwards to suit your needs, so you can quickly have a working
parser and tune it later.

The inference is done by the following rules:

- Each grammar rule

### Passing `Context` to actions

The default builder can be configured to pass lexing context to actions. This
can be used to write semantic actions which utilize lexing information like
position or surrounding content. For example, layout parser used with string
lexer uses this to construct and return a borrowed string slice which span the
layout preceding a next valid token. To be able to return a string slice layout
actions need access to the input string and start/end positions.

By default `Context` is not passed to actions. You can change that by
`RustemoSettings::pass_context(true)` which is also controlled by
`--pass-context` switch in `rustemo` CLI.

When this settings is in action, for the grammar:

```
{{#include ../../tests/src/pass_context/pass_context.rustemo}}
```

we get these generated actions:
```rust
{{#include ../../tests/src/pass_context/pass_context_actions.rs}}
```

```admonish note
The example above is from the [pass context test](https://github.com/igordejanovic/rustemo/tree/main/tests/src/pass_context) where actions are manually
modified to perform the position based calculation. The actual generated actions
will be slightly different but the main point is that each action function now
have an additional parameter of type `Context`.
```

We can see that in `num` action we are adding `context.position` to the parsed number.

The result of this test:

```rust
{{#include ../../tests/src/pass_context/mod.rs:pass_context}}
```

is:

```rust
{{#include ../../tests/src/pass_context/pass_context.ast}}
```

Where we get `3` as parsed `1` plus its position `2`, and we get `46` as parsed
`42` plus its position `4`.



## Generic tree builder

This is a built-in builder that will produce a generic parse tree.

For example, given the grammar:

```
{{#include ../../tests/src/generic_tree/generic_tree.rustemo}}
```

The test:

```rust
{{#include ../../tests/src/generic_tree/mod.rs:generic_tree}}
```

Will produce this output:

```
{{#include ../../tests/src/generic_tree/generic_tree.ast}}
```

You can see that each node in the tree is a `TermNode` or `NonTermNode` variant
of `TreeNode` enum.

For details see [the full
test](https://github.com/igordejanovic/rustemo/tree/main/tests/src/generic_tree).

```admonish note
Generic builder can be configured by `RustemoSettings::builder_type(BuilderType::Generic)`
settings API, exposed through `--builder-type generic` in the `rustemo` CLI.
```


## Custom builders

If you have a specific requirement for the build process you can implement a
builder from scratch.

To provide a custom builder you start with a type that implements a
`rustemo::builder::Builder` trait and after that implements a concrete parsing
algorithm trait. Currently, Rustemo is a LR parser thus you can use
`rustemo::builder::lr::Builder` trait.

Let's see how can we do all of this by implementing a builder that does
on-the-fly calculation of the arithmetic expression. Start with a type and a
base `Builder` trait implementation as each builder needs initialization and
should be able to return the final result:

```rust
{{#include ../../tests/src/custom_builder/custom_builder_builder.rs:custom-builder-base}}
```


Now, implement LR part of the builder. For this we must specify what should be
done for `shift/reduce` operations:

```rust
{{#include ../../tests/src/custom_builder/custom_builder_builder.rs:custom-builder-lr}}
```

```admonish tip
You can see the full test [here](https://github.com/igordejanovic/rustemo/tree/main/tests/src/custom_builder).
```


