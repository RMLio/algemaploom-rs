#!/usr/bin/env bash

set -e

# Rust build
echo "==> Rust building and bindings"
cargo build --lib --release --features=jni

# Java bindings
echo "==> Java bindings"
# Compile Translator CLI
javac src/java/Translator.java
