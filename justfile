# Set OFFLINE=true env variable for offline mode
nix_flags := if env("OFFLINE", "false") == "true" { "--offline" } else { "" }
cargo_flags := if env("OFFLINE", "false") == "true" { "--frozen" } else { "" }

export RUST_BACKTRACE := "1"
export CARGO_INCREMENTAL := "1"
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL := "sparse"

version := `grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/'`

# Show this help
help:
    @just --list --unsorted

# Check code issues with clippy
lint:
    nix develop {{nix_flags}} \
      --command cargo {{cargo_flags}} clippy --all --all-targets -- -D warnings

# Check code issues with clippy nightly
lint-nightly:
    nix develop {{nix_flags}} .#nightly \
      --command cargo {{cargo_flags}} clippy --all --all-targets -- -D warnings

# Run tests
test:
    # Default test
    nix {{nix_flags}} develop --command cargo {{cargo_flags}} nextest run
    # Test with array-based table generator
    nix {{nix_flags}} develop --command cargo nextest {{cargo_flags}} run --features arrays

# Run tests for the compiler
test-compiler:
    nix {{nix_flags}} develop --command cargo {{cargo_flags}} nextest run -p rustemo-compiler

# Run all checks
check: lint install-compiler doc-examples test

# Run all tests for all versions. Run with -L flag to see full output.
check-all flags="":
    nix {{nix_flags}} flake check {{flags}}

# Format the code
[no-cd]
format *paths=".":
    nix {{nix_flags}} develop --command cargo {{cargo_flags}} fmt --all {{paths}}

# Install the rcomp compiler
install-compiler: test-compiler
    nix {{nix_flags}} develop --command cargo {{cargo_flags}} install --path rustemo-compiler --debug

# Setup development environment with STABLE version
stable:
    nix {{nix_flags}} develop .#default

# Setup development environment with NIGHTLY version
nightly:
    nix {{nix_flags}} develop .#nightly

# Serve docs locally
docs:
    python -c "import webbrowser; webbrowser.open('http://localhost:3000/')"
    cd docs && nix {{nix_flags}} develop --command mdbook serve

# Compile docs examples
doc-examples: install-compiler
    rcomp docs/src/readme_example/src/testlr/calclr.rustemo
    rcomp --parser-algo glr docs/src/readme_example/src/testglr/calc.rustemo
    for i in $(seq 1 5); do \
      rcomp docs/src/tutorials/calculator/calculator$$i/src/calculator.rustemo; \
    done

# Login to crates.io and cache the API key - needed for release
login:
    @echo "Version: {{version}}"
    cargo login

# Dry run test releasing rustemo crate to crates.io
[private]
test-release-rustemo:
    cargo publish --dry-run -p rustemo
    cargo package --list -p rustemo
    @echo "Ready to publish rustemo version {{version}}?"
    @read -p "Type 'y' to confirm release: " CONFIRM; \
    if [ "$CONFIRM" != "y" ]; then \
        echo "Release aborted."; \
        exit 1; \
    fi

# Release rustemo crate to crates.io
release-rustemo: test-release-rustemo
    cargo publish -p rustemo

# Dry run test releasing rustemo compiler crate to crates.io
[private]
test-release-compiler:
    cargo publish --dry-run -p rustemo-compiler
    cargo package --list -p rustemo-compiler
    @echo "Ready to publish rustemo compiler version {{version}}?"
    @read -p "Type 'y' to confirm release: " CONFIRM; \
    if [ "$CONFIRM" != "y" ]; then \
        echo "Release aborted."; \
        exit 1; \
    fi

# Release rustemo compiler crate to crates.io
release-compiler: test-release-compiler
    cargo publish -p rustemo-compiler

# Tag a new release and push to GitHub
release-tag-push:
    git tag -s {{version}} -m "Release {{version}}"
    git push
    git push --tags origin {{version}}
    @echo "Don't forget to make GitHub release"
