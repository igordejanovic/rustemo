{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		mdbook-theme = {
			url = "github:zjp-CN/mdbook-theme";
			flake = false;
		};
  };

  outputs = { self, nixpkgs, flake-utils, mdbook-theme }: 
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
				buildInputs = with pkgs; [ wget git mdbook mdbook-admonish mdbook-plantuml mdbook-graphviz mdbook-theme-pkg graphviz mdbook-linkcheck ];
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
