#!/usr/bin/env sh
set -e -u

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=1
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse


cargo test -p rustemo-compiler
cargo install --path rustemo-compiler --debug

cd docs/src/tutorials/calculator/
for i in {1..5}; do
    rcomp calculator$i/src/calculator.rustemo;
done

cd -
# Deafult test
cargo test
# Test with function based table generator
cargo test --features functions
cargo clippy --all --all-targets -- -D warnings
cargo fmt --all
