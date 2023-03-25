# Bootstrapping

This section describes the bootstrap process which is essential to understand in
order to contribute to the development of the Rustemo library.

It is usual for compiler compilers to be implemented using themselves. Rustemo
is no different. in `rustemo/src/lang` you can find grammar `rustemo.rustemo`
which is a description of the rustemo grammar language. This description is then
used to generate a parser for rustemo grammar files.

The problem with bootstrapping is a classical chicken and egg problem. To
generate the parser you need a working parser. The problem is solved by using a
previous version to generate the next.

While the solution seems simple it is not easily achieved from the
organizational point of view. E.g., when you change parser generator code you
would like to have rustemo parser regenerated with the new code but the current
parser version might not be functional at that point.

Thus, rustemo defines a bootstrapping process to help with the development. The
idea is to build bootstrapping rustemo binary with the parser code from the git
`main` branch and the rest of the code from the current source tree.

If you are not changing the rustemo grammar or the parser code generator you
won't need bootstrapping and should proceed as usual with Cargo commands.

But, if you do need to change the rustemo grammar or parser code generator you
should install bootstrapping binary with the following command.

    $ cargo install --path rustemo-compiler --features bootstrap --debug

The `--debug` switch is optional but will provide faster build and the built
binary will provide better error reports in case of problems.

Note the use of `--features bootstrap`. This command will checkout rustemo
parser files (parser and actions) from the git `main` branch, do the build with
the rest of the code and install the binary.

You can verify that the bootstrapping binary is used by checking the version:

    $ rustemo --version
    rustemo 0.1.0-1a45d75ca4-bootstrap

```admonish note
It is assumed that the `main` branch contains a working parser.
```

When the bootstrapping binary is installed you develop as usual and run tests:

    $ cargo test

Whenever you change the rustemo grammar you should regenerate the parser code
with `rustemo` binary:

    rustemo rustemo/src/lang/rustemo.rustemo

If bootstrapping binary is used, code generation templates from the working tree
when the binary was last built are used. Thus, regenerate bootstrapping binary
whenever you change parser code generation templates.

This will also check your grammar for syntax and semantic errors and report
diagnostics.

If feature `bootstrap` is not provided during rustemo installation, Cargo
proceeds as usual by using current versions of parser files to build the binary.
