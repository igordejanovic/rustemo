{ pkgs, crane, mdbook-theme }:
let
	inherit (pkgs) stdenv lib;
	
	mdbook-theme-pkg = pkgs.rustPlatform.buildRustPackage {
		pname = "mdbook-theme";
		version = "0.1.4";
		cargoLock.lockFile = mdbook-theme.outPath + "/Cargo.lock";
		src = mdbook-theme;
	};

	craneLib = crane.mkLib pkgs;

	tex = pkgs.texlive.combine {
		inherit (pkgs.texlive) scheme-small standalone qtree pict2e preview;
	};

	buildInputs = with pkgs; [
		wget git bash
		mdbook mdbook-admonish mdbook-plantuml
		mdbook-graphviz mdbook-theme-pkg mdbook-linkcheck
		plantuml graphviz tex poppler_utils];

	docsFileTypes = [ ".*md$" ".*rustemo$" ".*err$" ".*ast$" ".*tex$" ".*png$" ".*css$" ".*js$" ".*sh$"];
	docsFilter = path: _type: builtins.any (pattern: builtins.match pattern path != null) docsFileTypes;
	docsOrCargoFilter = path: type:
		(docsFilter path type) || (craneLib.filterCargoSources path type);

	book = stdenv.mkDerivation {
		name = "rustemo-book";
		src = lib.cleanSourceWith {
			src = ../.;
			filter = docsOrCargoFilter;
		};
		inherit buildInputs;

		buildPhase = ''
			mdbook build docs
		'';
		installPhase = ''
			mv docs/book $out
		'';
	};
in
{
	inherit buildInputs;
	packages = { inherit book; };
}
