# Bootstrapping

It is usual for compiler compilers to be implemented using themselves. Rustemo
is no different. in `rustemo/src/lang` you can find grammar `rustemo.rustemo`
which is a description of the rustemo grammar language. This description is then
used to generate a parser for rustemo grammar files.

The problem with bootstrapping is a classical chicken and egg problem. To
generate the parser you need a working parser. The problem is solved by using a
previous version to generate the next.

While the solution seems simple it is not easily achieved from the
organizational point of view. Thus, rustemo defines a bootstrapping process to
help with the development.

If you are not changing the rustemo grammar or the parse code generator you
won't need bootstrapping and should proceed as usual with Cargo commands.

But, if you do need to change rustemo grammar or parser code generator you can
start the bootstrapping process.

For this purpose there is a CLI tool `boostrap` in the project repo. Install
this tool with:

```sh
$ cargo install --path bootstrap
```

Now, you have this little tool which is used to start/finish bootstrapping process.

To start bootstrapping:

```
$ bootstrap start
```

This command will checkout rustemo parser files from the `main` branch of the
git repo and then build the bootstrapping rustemo binary. You can verify that
the bootstrapping binary is used by:

```
$ target/debug/rustemo --version
rustemo 0.1.0-bootstrap
```

```admonish 
It is assumed that you are calling the command anywhere from the git repo and 
that the `main` branch contains a working parser.
```

These bootstrapping files are available in the same folder where the original
parser is (`rustemo/src/lang`) but with `_bootstrap` suffix. If for whatever
reason you want to regenerate bootstrapping binary (e.g. maybe you have changed
parser code generator and want those changes to be applied) you do that by
calling:

```sh
$ cargo build -p rustemo --features bootstrap
```

When the bootstrapping binary is in place you develop as usual and run tests:

```sh
$ cargo test
```

If feature `bootstrap` is not provided, like in the previous command Cargo
proceeds as usual by using current versions of parser files to build the test
binary and run the tests. But, before the test binary build, the `build.rs`
script will re-generate parser files from the grammar using the bootstrap
binary.

When you have finished your changes and all the tests pass you can terminate the
bootstrap process with:

```sh
$ bootstrap finish
```

This command will simply delete the `_bootstrap` files. You can do that manually
if you wish.

