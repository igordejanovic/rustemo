{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs = {
				nixpkgs.follows = "nixpkgs";
				flake-utils.follows = "flake-utils";
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

	outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, mdbook-theme }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};

				rev = if (self ? rev) then self.rev else self.dirtyRev;
				book = import ./docs {
					inherit pkgs crane mdbook-theme;
				};
				rustemo = import ./. {
					inherit crane pkgs rev;
				};
				shellPkgs = [ pkgs.cargo-nextest rustemo.packages.compiler ];
			in
			{
				devShells.default = pkgs.mkShell {
					buildInputs = book.buildInputs ++ rustemo.buildInputs ++ shellPkgs;
				};
				devShells.beta = pkgs.mkShell {
					buildInputs = book.buildInputs ++ [ pkgs.rust-bin.beta.latest.default ] ++ shellPkgs;
				};
				devShells.nightly = pkgs.mkShell {
					buildInputs = book.buildInputs ++ [ pkgs.rust-bin.nightly.latest.default ] ++ shellPkgs;
				};
				inherit (rustemo) checks;
				packages = rustemo.packages // book.packages;
			}
		);

	nixConfig = {
		bash-prompt-prefix = "rustemo-dev:";
	};
}
