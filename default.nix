{ crane, pkgs, rev }:
let
	inherit (pkgs) stdenv lib;

	src = lib.cleanSource ./.;
	inherit (let craneLib = crane.mkLib pkgs; in craneLib.crateNameFromCargoToml { inherit src; }) version;
	
	packageArtifactsForLib = craneLib: args: rec {
			commonArgs = args // {
				inherit src;
				strictDeps = true;
			};
			cargoArtifacts = craneLib.buildDepsOnly commonArgs;
	};
	buildPackageForToolchain = toolchain:
		let 
			craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
		in pname:
			let
				inherit (packageArtifactsForLib craneLib { inherit pname; }) commonArgs cargoArtifacts;
				buildArgs = commonArgs // {
					inherit cargoArtifacts version;
					cargoExtraArgs = "-p ${pname}";
					GIT_HASH = rev;
				};
			in craneLib.buildPackage buildArgs;

	checksForPackage = { craneLib, commonArgs, cargoArtifacts }:
		let
			tests = craneLib.cargoNextest (commonArgs // {
				inherit cargoArtifacts;
			});
			clippy = craneLib.cargoClippy (commonArgs // {
				inherit cargoArtifacts;
				cargoClippyExtraArgs = "--all-targets -- --deny warnings";
			});
			fmt = craneLib.cargoFmt commonArgs;
		in stdenv.mkDerivation {
			name = "${commonArgs.pname}-check";
			buildInputs = [ tests clippy fmt ];
			dontUnpack = true;
			installPhase = ''
				touch $out
			'';
		};

	workspaceChecksForToolchain = toolchain:
		let
			craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
			checksGenerationArgs = pname: 
			let
				inherit (packageArtifactsForLib craneLib { inherit pname; }) commonArgs cargoArtifacts;
			in {
				inherit craneLib cargoArtifacts;
				commonArgs = commonArgs // { GIT_HASH = rev; };
			};
			pnames = [ 
				"rustemo"
				"rustemo-compiler"
				"rustemo-tests"
				"calculator"
				"calculator1"
				"calculator2"
				"calculator3"
				"calculator4"
				"calculator5"
				"expressions"
				"readme_example"
			];
		in stdenv.mkDerivation {
			name = "rustemo-workspace-check";
			buildInputs = map (pname: checksForPackage (checksGenerationArgs pname)) pnames;
			dontUnpack = true;
			GIT_HASH = rev;
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
