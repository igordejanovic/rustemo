# Installation and setup

To render this document locally in your browser do (this should be done only
once to install `mdbook`):

1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. With cargo install mdbook and used processors
   ```sh
   cargo install mdbook --git https://github.com/igordejanovic/mdBook.git --branch merged-prs
   cargo install mdbook-admonish
   cargo install mdbook-bib
   cargo install mdbook-theme
   cargo install mdbook-plantuml
   cargo install mdbook-graphviz
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

In the root of the `docs` directory run `mdbook serve`. The book will be available
at [http://localhost:3000/]()

# Setup using Nix Flakes

Alternatively, you can use [Nix package manager](https://nixos.org/) to setup
documentation locally. Instead of the above steps just install Nix, [enable
flakes](https://nixos.wiki/wiki/flakes), and from the root of the project run:

```
nix develop
mdbook serve docs
```

Nix-based shell has an added benefix that it provides all additional required
dependencies (e.g. `pdflatex` and required package for building tree images,
plantuml, graphviz). To rebuild tree images from `.tex` files run
`docs/build-latex-images.sh` (See [Trees diagrams](#trees-diagrams) bellow).
  
# Admonitions

This doc uses `mdbook-admonish` pre-processor. See [here](https://tommilligan.github.io/mdbook-admonish/).

# Including files

Including files ensure that the docs is up-to-date with the content it is
referring to.

See [this part of the manual](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).

# Cross references

See [this](https://users.rust-lang.org/t/mdbook-how-to-use-cross-references/83713) on how to reference to other section of the document.

# Bibliography

Uses [mdbook-bib](https://github.com/francisco-perez-sorrosal/mdbook-bib) to reference books, papers etc.

# Diagrams

For UML diagrams we use [PlantUML](https://plantuml.com/), while for general graphs and trees
[GraphViz](https://graphviz.org/) is used.

# Trees diagrams

For tree diagrams LaTeX (`pdflatex`) with `qtree` package is used to produce PDF
and afterwards the PDF file is converted to PNG using `pdftoppm`. See
`docs/build-latex-images.sh` script. This script must be called whenever `.tex`
files with trees description are changed.
