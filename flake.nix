{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mdbook-theme = {
      url = "github:zjp-CN/mdbook-theme";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      mdbook-theme,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustStable = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "clippy"
            "rustfmt"
            "rust-docs"
            "rust-analyzer"
          ];
        };

        rustNightly = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [
            "rust-src"
            "clippy"
            "rustfmt"
            "rust-docs"
            "rust-analyzer"
          ];
        };

        rev = if (self ? rev) then self.rev else self.dirtyRev;
        book = import ./docs {
          inherit pkgs crane mdbook-theme;
        };
        rustemo = import ./. {
          inherit crane pkgs rev;
        };
        shellPkgs = [
          pkgs.cargo-nextest
          rustemo.packages.compiler
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = book.buildInputs ++ rustemo.buildInputs ++ shellPkgs ++ [ rustStable ];
          RUST_SRC_PATH = "${rustStable}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [ pkgs.zlib ]}";
          shellHook = ''
            echo "Using STABLE Rust environment"
          '';
        };
        devShells.beta = pkgs.mkShell {
          buildInputs = book.buildInputs ++ [ pkgs.rust-bin.beta.latest.default ] ++ shellPkgs;
        };
        devShells.nightly = pkgs.mkShell {
          buildInputs = book.buildInputs ++ [ rustNightly ] ++ shellPkgs;
          RUST_SRC_PATH = "${rustNightly}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [ pkgs.zlib ]}";
          shellHook = ''
            echo "Using NIGHTLY Rust environment"
          '';
        };
        inherit (rustemo) checks;
        packages = rustemo.packages // book.packages;
      }
    );

  nixConfig = {
    bash-prompt-prefix = "rustemo-dev:";
  };
}
