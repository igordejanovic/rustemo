{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
		
		crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    mdbook-theme = {
      url = "github:zjp-CN/mdbook-theme";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, mdbook-theme }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

				book = import ./docs {
					inherit pkgs mdbook-theme;
				};
				rustemo = import ./. {
					inherit crane pkgs;
					rev = self.dirtyRev;
				};
			in
      {
        devShells.default = pkgs.mkShell { buildInputs = book.buildInputs ++ rustemo.buildInputs; };
				inherit (rustemo) checks;
				packages = rustemo.packages // book.packages;
			}
    );
}
