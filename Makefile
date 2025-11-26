.PHONY: help clean clean-test clean-pyc clean-build \
	lint test check check-all login release-test release \
	test-compiler install-compiler dev docs
.DEFAULT_GOAL := help

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=1
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
VERSION := $(shell grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

define BROWSER_PYSCRIPT
import webbrowser, sys
webbrowser.open(sys.argv[1])
endef
export BROWSER_PYSCRIPT

define PRINT_HELP_PYSCRIPT
import re, sys

for line in sys.stdin:
	match = re.match(r'^([a-zA-Z_-]+):.*?## (.*)$$', line)
	if match:
		target, help = match.groups()
		print("%-20s %s" % (target, help))
endef
export PRINT_HELP_PYSCRIPT
BROWSER := python -c "$$BROWSER_PYSCRIPT"

help:
	@python -c "$$PRINT_HELP_PYSCRIPT" < $(MAKEFILE_LIST)

lint:  ## check code issues with clippy nightly
	nix develop .#nightly --command cargo clippy --all --all-targets -- -D warnings

test:  ## run tests
    # Default test
	nix develop --command cargo nextest run
    # Test with array-based table generator
	nix develop --command cargo nextest run --features arrays

format:  ## Format the code
	nix develop --command cargo fmt --all

doc-examples:  install-compiler  ## compile docs examples
	rcomp docs/src/readme_example/src/testlr/calclr.rustemo
	rcomp --parser-algo glr docs/src/readme_example/src/testglr/calc.rustemo

	for i in {1..5}; do \
		rcomp docs/src/tutorials/calculator/calculator$$i/src/calculator.rustemo; \
	done

check: lint install-compiler doc-examples test  ## Run all checks

check-all:  ## Run all tests for all versions
	nix flake check

login:  ## login to crates.io and caches the API key - needed for release
	@echo "Version: $(VERSION)"
	cargo login

release-test: 	## dry run test releasing crates to crates.io
    # Running dry run for rustemo package
	cargo publish --dry-run -p rustemo	
	cargo package --list -p rustemo
    # Running dry run for rustemo package
	cargo publish --dry-run -p rustemo	
	cargo package --list -p rustemo

release:  ## release packages to crates.io and create git tag
    # publish
	cargo publish -p rustemo	
	cargo publish -p rustemo-compiler
	git tag -s $(VERSION) -m "Release $(VERSION)"
	git push
	git push origin $(VERSION)
	@echo "Don't forget to make GitHub release"

test-compiler:  ## run tests for the compiler
	nix develop --command cargo nextest run -p rustemo-compiler

install-compiler:  test-compiler  ## install the rcomp compiler
	nix develop --command cargo install --path rustemo-compiler --debug

dev:  ## Setup development environment
	nix develop .#default

nightly:  ## Setup development environment with nightly version
	nix develop .#nightly

docs:  ## Serve docs locally
	$(BROWSER) "http://localhost:3000/"
	(cd docs && nix develop --command mdbook serve)
