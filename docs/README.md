# Installation and setup

To render this document locally in your browser do (this should be done only
once to install `mdbook`):

1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. With cargo install mdbook and used processors
   ```sh
   cargo install mdbook 
   cargo install mdbook-admonish
   cargo install mdbook-bib
   cargo install mdbook-theme
   cargo install mdbook-plantuml
   cargo install mdbook-linkcheck
   ```

Optional:

1. For pdf output:
   ```sh
   cargo install mdbook-pdf
   ```
   
2. For pdf outline support see [this](https://github.com/HollowMan6/mdbook-pdf/issues/1#issuecomment-1366157949):
   ```sh
   pip install --user mdbook-pdf-outline
   ```

In the root of the `docs` folder run `mdbook serve`. The book will be available at [http://localhost:3000/]()
  
# Admonitions

This doc uses `mdbook-admonish` pre-processor. See
[here](https://tommilligan.github.io/mdbook-admonish/).

# Including files

Including files ensure that the docs is up-to-date with the content it is referring to.

See [this part of the
manual](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).

# Cross references

See
[this](https://users.rust-lang.org/t/mdbook-how-to-use-cross-references/83713)
on how to reference to other section of the document.

# Bibliography

Uses [mdbook-bib](https://github.com/francisco-perez-sorrosal/mdbook-bib) to reference books, papers etc.
