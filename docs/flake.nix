{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		mdbook-theme = {
			url = "github:zjp-CN/mdbook-theme";
			flake = false;
		};
		# mdbook with fixes.
		# Current fixes:
		# - https://github.com/rust-lang/mdBook/pull/1718
		mdbook = rec {
			url = "github:igordejanovic/mdbook?ref=merged-prs";
			flake = false;
		};
  };

  outputs = { self, nixpkgs, flake-utils, mdbook-theme, mdbook }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = nixpkgs.legacyPackages.${system};
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
				buildInputs = with pkgs; [ wget git
																	 mdbook-pkg mdbook-admonish mdbook-plantuml
																	 mdbook-graphviz mdbook-theme-pkg mdbook-linkcheck
																	 plantuml graphviz
																	 tex
																	 poppler_utils];
			in
			{
				devShells.default = mkShell { inherit buildInputs; };
				packages.default = stdenv.mkDerivation {
					name = "rustemo-book";
					src = ./.;
					inherit buildInputs;

					buildPhase = ''
						mdbook build
					'';
					installPhase = ''
						mv book $out
					'';
				};
			}
		);
}
