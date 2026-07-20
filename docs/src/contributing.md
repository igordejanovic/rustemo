# Contributing

Contributions are welcome, and they are greatly appreciated!. You can contribute
code, documentation, tests, bug reports. Every little bit helps, and credit will
always be given. If you plan to make a significant contribution it would be
great if you first announce that in [the
Discussions](https://github.com/igordejanovic/rustemo/discussions).


## Legal Notice

When contributing to this project, you must agree that you have authored 100% of
the content, that you have the necessary rights to the content and that the
content you contribute may be provided under the project license.


## Guidelines for AI-assisted Contributions

AI tools are welcome as helpers, not authors. Keep these practices in mind:

- Stay accountable: only submit changes you understand and can justify; be ready
  to explain behavior, edge cases, and alignment with Rustemo conventions. If
  an AI suggestion feels unclear, rewrite or drop it.
- Keep humans in the loop: discuss non-trivial ideas early via Issues or
  Discussions, especially when you are unsure about design or impact.
- Use AI for acceleration, then verify: treat AI output as a draft for code,
  tests, or docs; run linters/tests and review the logic yourself.
- Be transparent in PRs: note briefly if AI was used and for what (e.g., initial
  draft, test scaffolding), and call out any parts where you want extra review.
- Prefer focused patches over large dumps; if you cannot confidently explain an
  AI-produced change, open a well-described issue instead.


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
just stable
```

in the root of the project. You will get a bash shell with all the required
dependencies installed for testing, building and working with the docs.

For easier project management Rustemo is using
[just](https://github.com/casey/just) command runner.

To see all recipes run:

```
just
```

To run documentation in server mode:

```
just docs
```

The docs will be available at http://localhost:3000/ and refreshed on each change.

To run all tests:

```
just test
```
  
To test everything, format the code, run the linter:

```
just check
```

To run full checks, as they are run in the CI:

```
just check-all
```

This full check takes a while as the tests are run for 3 versions of Rust. To
see the progress in more details you can run `check-all` with `-L` flag:

```
just check-all -L
```

Please do note that it is a requirement for full checks to pass for the
contribution to be merged to the main branch. But, you don't have to run it
locally. When you make a Pull Request, GitHub CI will run them.

To get a specific Rust version environment you can do one of:

```
just stable
just nightly
```

