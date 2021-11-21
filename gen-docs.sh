#!/bin/sh
# Insert custom html header for katex usage
RUSTDOCFLAGS="--html-in-header docs-header.html" cargo doc --no-deps --document-private-items $1
