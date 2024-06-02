{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";

    mdbook-theme = {
      url = "github:zjp-CN/mdbook-theme";
      flake = false;
    };
    # mdbook with fixes.
    # Current fixes:
    # - https://github.com/rust-lang/mdBook/pull/1718
    mdbook = {
      url = "github:igordejanovic/mdbook?ref=merged-prs";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, mdbook-theme, mdbook }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) stdenv mkShell;

        mdbook-theme-pkg = pkgs.rustPlatform.buildRustPackage {
          pname = "mdbook-theme";
          version = "0.1.4";
          cargoLock.lockFile = mdbook-theme.outPath + "/Cargo.lock";
          src = mdbook-theme;
        };

        mdbook-pkg = pkgs.rustPlatform.buildRustPackage {
          pname = "mdbook";
          version = "0.4.26.fix";
          cargoLock.lockFile = mdbook.outPath + "/Cargo.lock";
          src = mdbook;
        };

        tex = pkgs.texlive.combine {
          inherit (pkgs.texlive) scheme-minimal standalone qtree;
        };

        buildInputsDocs = with pkgs; [
          wget git bash
          mdbook-pkg mdbook-admonish mdbook-plantuml
          mdbook-graphviz mdbook-theme-pkg mdbook-linkcheck
          plantuml graphviz tex poppler_utils];

        devInputs = with pkgs; buildInputsDocs ++ [
           rust-bin.stable.latest.default
        ];
      in
      {
        devShells.default = mkShell { buildInputs = devInputs; };
        packages.default = stdenv.mkDerivation {
          name = "rustemo-book";
          src = ./.;
          buildInputs = buildInputsDocs;

          buildPhase = ''
             mdbook build docs
          '';
 					installPhase = ''
 						 mv docs/book $out
 					'';
        };
      }
    );
}
