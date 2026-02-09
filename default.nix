{
  crane,
  pkgs,
  rev,
}:
let
  inherit (pkgs) stdenv lib;

  # Minimal supported Rust version
  msrv = "1.82.0";

  craneLib = crane.mkLib pkgs;

  rustemoFileTypes = [
    ".rustemo"
    ".err"
    ".ast"
    ".json"
    ".bytes"
    ".calc"
  ];
  rustemoFilter = path: _type: builtins.any (ext: lib.hasSuffix ext path) rustemoFileTypes;
  rustemoOrCargoFilter =
    path: type: (rustemoFilter path type) || (craneLib.filterCargoSources path type);

  src = lib.cleanSourceWith {
    src = craneLib.path ./.;
    filter = rustemoOrCargoFilter;
  };

  inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;

  commonArgs = {
    inherit src;
    strictDeps = true;
    doCheck = false;
    GIT_HASH = rev;
  };

  # Build *just* the cargo dependencies (of the entire workspace),
  # so we can reuse all of that work (e.g. via cachix) when running in CI
  # It is *highly* recommended to use something like cargo-hakari to avoid
  # cache misses when building individual top-level-crates
  cargoArtifactsForToolchain =
    { toolchain, name }:
    let
      craneLibToolchain = craneLib.overrideToolchain toolchain;
    in
    craneLibToolchain.buildDepsOnly (
      commonArgs
      // {
        pname = "rustemo-${name}-workspace";
      }
    );

  workspaceChecksForToolchain =
    { toolchain, name }:
    let
      craneLibToolchain = craneLib.overrideToolchain toolchain;
      pname = "rustemo-${name}-workspace";
      cargoArtifacts = cargoArtifactsForToolchain { inherit toolchain name; };
      baseArgs = commonArgs // {
        inherit cargoArtifacts pname;
      };
      tests = craneLibToolchain.cargoNextest (baseArgs // { doCheck = true; });
      clippy = craneLibToolchain.cargoClippy (
        baseArgs
        // {
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        }
      );
      fmt = craneLibToolchain.cargoFmt (baseArgs // { inherit pname; });
    in
    stdenv.mkDerivation {
      name = "${pname}-check";
      # A convenience for running each individual check when needed from CLI.
      # E.g.: nix build .#checks.x86_64-linux.stable.clippy
      # Until this is solved: https://github.com/NixOS/nix/issues/8881
      inherit clippy tests fmt;
      buildInputs = [
        clippy
        tests
        fmt
      ];
      dontUnpack = true;
      installPhase = ''
        touch $out
      '';
    };

  buildPackageForToolchain =
    toolchain: pname:
    let
      craneLibToolchain = craneLib.overrideToolchain toolchain;
    in
    craneLibToolchain.buildPackage (
      commonArgs
      // {
        cargoArtifacts = cargoArtifactsForToolchain {
          inherit toolchain;
          name = "stable";
        };
        inherit version pname;
        cargoExtraArgs = "-p ${pname}";
      }
    );

  wasmCheck =
    toolchain:
    let
      craneLibToolchain = craneLib.overrideToolchain (
        toolchain.override {
          targets = [ "wasm32-unknown-unknown" ];
        }
      );
    in
    craneLibToolchain.buildPackage {
      src = lib.cleanSourceWith {
        src = craneLib.path ./.;
        filter = rustemoOrCargoFilter;
      };
      pname = "rustemo-wasm";
      version = "0.0.0";
      strictDeps = true;
      doCheck = false;
      cargoExtraArgs = "--target wasm32-unknown-unknown --manifest-path test-wasm/Cargo.toml";
      CARGO_TARGET_DIR = "target";
    };
in
{
  inherit msrv;
  checks = with pkgs.rust-bin; {
    base = workspaceChecksForToolchain {
      toolchain = stable.${msrv}.default;
      name = "base";
    };
    beta = workspaceChecksForToolchain {
      toolchain = beta.latest.default;
      name = "beta";
    };
    nightly = workspaceChecksForToolchain {
      toolchain = nightly.latest.default;
      name = "nightly";
    };
    stable = workspaceChecksForToolchain {
      toolchain = stable.latest.default;
      name = "stable";
    };
    wasm = wasmCheck stable.latest.default;
  };
  packages = rec {
    default = compiler;
    compiler = buildPackageForToolchain pkgs.rust-bin.stable.latest.default "rustemo-compiler";
  };
}
