#!/usr/bin/env sh
set -e -u

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=1
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Test linter issues first
cargo clippy --all --all-targets -- -D warnings

# We must first test and install compiler itself
cargo nextest run -p rustemo-compiler
cargo install --path rustemo-compiler --debug

# README Examples
rcomp docs/src/readme_example/src/textlr/calclr.rustemo
rcomp --parser-algo glr docs/src/readme_example/src/textglr/calc.rustemo

cd docs/src/tutorials/calculator/
for i in {1..5}; do
    rcomp calculator$i/src/calculator.rustemo;
done

cd -

# Default test
cargo nextest run

# Test with array-based table generator
cargo nextest run --features arrays

cargo fmt --all
