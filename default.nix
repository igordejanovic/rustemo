{ crane, pkgs, rev }:
let
	inherit (pkgs) stdenv lib;

	craneLib = crane.mkLib pkgs;

	rustemoFileTypes = [ ".rustemo" ".err" ".ast" ];
	rustemoFilter = path: _type: builtins.any (ext: lib.hasSuffix ext path) rustemoFileTypes;
	rustemoOrCargoFilter = path: type:
		(rustemoFilter path type) || (craneLib.filterCargoSources path type);

	src = lib.cleanSourceWith {
		src = craneLib.path ./.;
		filter = rustemoOrCargoFilter;
	};

	inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;

	commonArgs = {
		inherit src;
		strictDeps = true;
		pname = "rustemo-workspace";
		doCheck = false;
		GIT_HASH = rev;
	};

	# Build *just* the cargo dependencies (of the entire workspace),
	# so we can reuse all of that work (e.g. via cachix) when running in CI
	# It is *highly* recommended to use something like cargo-hakari to avoid
	# cache misses when building individual top-level-crates
	cargoArtifactsForToolchain = toolchain:
		let
			craneLibToolchain = craneLib.overrideToolchain toolchain;
		in {
			cargoArtifacts = craneLibToolchain.buildDepsOnly commonArgs;
		};


	buildPackageForToolchain = toolchain:
		let 
			craneLibToolchain = craneLib.overrideToolchain toolchain;
		in pname:
			let
				buildArgs = commonArgs // {
					inherit (cargoArtifactsForToolchain toolchain) cargoArtifacts;
					inherit version;
					cargoExtraArgs = "-p ${pname}";
				};
			in craneLibToolchain.buildPackage buildArgs;

	workspaceChecksForToolchain = toolchain:
		let
			craneLibToolchain = craneLib.overrideToolchain toolchain;
			tests = craneLibToolchain.cargoNextest (commonArgs // {
				inherit (cargoArtifactsForToolchain toolchain) cargoArtifacts;
				doCheck = true;
			});
			clippy = craneLibToolchain.cargoClippy (commonArgs // {
				inherit (cargoArtifactsForToolchain toolchain) cargoArtifacts;
				cargoClippyExtraArgs = "--all-targets -- --deny warnings";
			});
			fmt = craneLibToolchain.cargoFmt commonArgs;
		in stdenv.mkDerivation {
			name = "rustemo-workspace-check";
			buildInputs = [ tests clippy fmt ];
			dontUnpack = true;
			installPhase = ''
				touch $out
			'';
		}; 			
in
{
	buildInputs = [ pkgs.rust-bin.stable.latest.default ];
	checks = with pkgs.rust-bin; {
		stable = workspaceChecksForToolchain stable.latest.default;
		beta = workspaceChecksForToolchain beta.latest.default;
		nightly = workspaceChecksForToolchain nightly.latest.default;
	};
	packages = rec {
		default = compiler;
		compiler = buildPackageForToolchain pkgs.rust-bin.stable.latest.default "rustemo-compiler";
	};
}
