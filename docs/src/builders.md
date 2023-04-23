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
There are two approaches where generated files are stored. See [the configuration section](./configuration.md).
```

### AST type inference

Based on grammar rules Rustemo will infer and generate AST node types which you
can modify afterwards to suit your needs, so you can quickly have a working
parser and tune it later.

The inference is done by the following rules:

Each non-terminal grammar rule will be of an `enum` type. The enum variants will
be:
1. If only non-content matches are in the production (e.g. just string matches)
   -> plain variant without inner content
2. A single content match (e.g. regex match) or rule reference without
   assignment -> variant with referred type as its content.
3. Multiple content matches and/or assignments -> a `struct` type where fields
   types are of the referred symbols.

In addtion, production kinds and assignments LHS names are used for
fields/function/type naming. Also, cycle refs are broken using `Box`.

Probably the best way to explain is by using an example. For example, if we have
the following grammar:

```
{{#include ../../examples/calculator/src/ast_actions/calculator04_ambig_lhs.rustemo}}
```

we get these generated actions/types:
```rust
{{#include ../../examples/calculator/src/ast_actions/calculator04_ambig_lhs_actions.rs}}
```

We see that each grammar rule will have its corresponding type defined. Also,
each production and each terminal will have an actions (Rust function)
generated. You can change these manually and your manual modifications will be
preserved on the next code generation as long as the name is the same.

```admonish tip
On each code generation the existing `<>_actions.rs` file is parsed using [syn
crate](https://docs.rs/syn/latest/syn/) and each type and action that is missing
in the existing file is regenerated at the end of the file while the existing
items are left untouched. The only items that cannot be preserved are non-doc
comments.

This enables you to regenerate types/actions by just deleting them from the
actions file and let rustemo run. If you want to regenerate actions/types from
scratch just delete the whole file.
```

Here is an example of generated and manually modified actions for the same grammar above:

```rust
{{#include ../../examples/calculator/src/calc_actions/calculator04_ambig_lhs_actions.rs}}
```

In these actions we are doing actual calculations. For the full explanation see
[the calculator tutorial](tutorials/calculator/calculator.md).

```admonish tip
Lexing context is passed to actions as a first parameter. This can be used to
write semantic actions which utilize lexing information like position or
surrounding content. For example, layout parser used with string lexer can use
this to construct and return a borrowed string slice which span the layout
preceding a next valid token. To be able to return a string slice, layout
actions need access to the input string and start/end positions.
```

## Generic tree builder

This is a built-in builder that will produce a generic parse tree (a.k.a
/Concrete-Syntax-Tree (CST)/).

For example, given the grammar:

```
{{#include ../../tests/src/builder/generic_tree/generic_tree.rustemo}}
```

The test:

```rust
{{#include ../../tests/src/builder/generic_tree/mod.rs:generic_tree}}
```

will produce this output:

```
{{#include ../../tests/src/builder/generic_tree/generic_tree.ast}}
```

We can see that we get all the information from the input. Each node in the tree
is a `TermNode` or `NonTermNode` variant of `TreeNode` enum. Each node keeps the
layout that precedes it.

For details see [the full
test](https://github.com/igordejanovic/rustemo/tree/main/tests/src/builder/generic_tree).

```admonish note
Generic builder can be configured by `Settings::new().builder_type(BuilderType::Generic)`
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
should be able to return the final result.

```admonish note
To be able to find custom builder Rustemo uses the following conventions:
- File name where builder is implemented is constructed by taking the filename
  of the grammar (moduo extension) and adding sufix `_builder`.
- Builder type name is constructed similarly by taking the name of the builder
  file and converting to CamelCase.
```

For example, given the grammar:

```rust
{{#include ../../tests/src/builder/custom_builder/custom_builder.rustemo}}

```

in file `custom_builder.rustemo` the following builder from file
`custom_builder_builder.rs` will perform arithmetic operation on-the-fly (during
parsing):

```rust
{{#include ../../tests/src/builder/custom_builder/custom_builder_builder.rs:custom-builder-base}}
```

Now, implement LR part of the builder. For this we must specify what should be
done for `shift/reduce` operations:

```rust
{{#include ../../tests/src/builder/custom_builder/custom_builder_builder.rs:custom-builder-lr}}
```

```admonish tip
You can see the full test [here](https://github.com/igordejanovic/rustemo/tree/main/tests/src/builder/custom_builder).
```
