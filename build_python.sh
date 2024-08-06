#!/usr/bin/env bash

set -e

# Rust build
echo "==> Rust building and bindings"
cargo build --lib --release --features=pyo3

# Python bindings
echo "==> Python bindings"
# Native import, but requires renaming
cp target/release/libltranslator.so src/python/ltranslator.so
