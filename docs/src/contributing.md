# Contributing

Contributions are welcome, and they are greatly appreciated!. You can contribute
code, documentation, tests, bug reports. Every little bit helps, and credit will
always be given. If you plan to make a significant contribution it would be
great if you first announce that in [the
Discussions](https://github.com/igordejanovic/rustemo/discussions).

You can contribute in many ways:

## Types of Contributions

1.  Report Bugs

    Report bugs at <https://github.com/igordejanovic/rustemo/issues>.
    
    If you are reporting a bug, please follow the instructions in the template.
    
2.  Fix Bugs

    Look through the GitHub issues for bugs. Anything tagged with `bug` and
    `help wanted` is open to whoever wants to implement it. If you are not sure
    how to proceed, you can ask in the corresponding issue.

3.  Implement Features

    Look through the GitHub issues for features. Anything tagged with
    `enhancement/feature` and `help wanted` is open to whoever wants to
    implement it.
    
4.  Write Documentation

    Rustemo could always use more documentation, whether as part of the official
    Rustemo docs, in documentation comments, or even on the web in blog posts,
    articles, and such.
    
    Rustemo is using [mdbook](https://github.com/rust-lang/mdBook) for the
    official documentation. Mdbook and its dependencies are handled by Nix (see
    the next section).

## Development setup

For contributing code and docs you are advised to do a proper setup. Rustemo
uses [Nix package manager](https://nixos.org/) for reproducible setups, tests
and builds.

After installing Nix and cloning the project all you have to do to start
developing is:

```
nix develop
```

in the root of the project. You will get a bash shell with all the required
dependencies installed for testing, building and working with the docs.

For example: 

- To run documentation in server mode:

    ```
    mdbook serve docs
    ```

    The docs will be available at http://localhost:3000/ and refreshed on each change.

- To run all tests:

  ```
  cargo nextest run
  ```
  
- To test everything, format the code, run the linter:

  ```
  ./check-update-all.sh
  ```


To run full checks, as they are run in the CI, use this command from your base
shell (not development shell from the above):

```
nix flake check
```

This full check takes a while as the tests are run for 3 versions of Rust. To
see the progress in more details you can run:

```
nix flake check -L
```

Please do note that it is a requirement for full checks to pass for the
contribution to be merged to the main branch. But, you don't have to run it
locally. When you make a Pull Request, GitHub CI will run them.
