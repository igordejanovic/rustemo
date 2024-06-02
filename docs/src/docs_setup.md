# Installation and setup

Rustemo uses [Nix package manager](https://nixos.org/) to setup reproducible
documentation development environment locally.

1. [Install Nix](https://nixos.org/download/)
2. [Enable flakes](https://nixos.wiki/wiki/flakes)

```
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

3. From the root of the project run:

```
nix develop
mdbook serve docs
```

The docs will be available at [http://localhost:3000/](http://localhost:3000/)
and the server will watch for changes and rebuild.

Nix-based shell has an added benefix that it provides all additional required
dependencies (e.g. `pdflatex` and required package for building tree images,
plantuml, graphviz). E.g., to rebuild tree images from `.tex` files you can run
`docs/build-latex-images.sh` (it is automatically called by `mdbook serve docs`,
see [Trees diagrams](#trees-diagrams) bellow).
  
# Admonitions

This doc uses `mdbook-admonish` pre-processor. See [here](https://tommilligan.github.io/mdbook-admonish/).

# Including files

Including files ensure that the docs is up-to-date with the content it is
referring to.

See [this part of the manual](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).

# Cross references

See [this](https://users.rust-lang.org/t/mdbook-how-to-use-cross-references/83713) on how to reference to other section of the document.

<!-- # Bibliography -->

<!-- Uses [mdbook-bib](https://github.com/francisco-perez-sorrosal/mdbook-bib) to reference books, papers etc. -->

# Diagrams

For UML diagrams we use [PlantUML](https://plantuml.com/), while for general graphs and trees
[GraphViz](https://graphviz.org/) is used.

# Trees diagrams

For tree diagrams LaTeX (`pdflatex`) with `qtree` package is used to produce PDF
and afterwards the PDF file is converted to PNG using `pdftoppm`. See
`docs/build-latex-images.sh` script. This script must be called whenever `.tex`
files with trees description are changed.
