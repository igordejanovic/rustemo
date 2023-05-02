#!/usr/bin/env sh

cargo test && \
    cargo install --path rustemo-compiler --debug && \
    cd docs/src/tutorials/calculator/ && \
    for i in {1..5}; do rcomp calculator$i/src/calculator.rustemo; done && \
    cd - && \
    cargo clippy && \
    cargo fmt --all
