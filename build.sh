#!/bin/sh
set -e

# Rust build
echo "==> Rust building and bindings"
cargo build

# Java bindings
echo "==> Java bindings"
# Compile Translator CLI
javac src/java/Translator.java

# NodeJS bindings
echo "==> NodeJS bindings"
# Install NodeJS NEON dependencies
npm i
# Execute NEON on Rust library to generate index.node file for NodeJS dynamic library
./node_modules/.bin/neon dist -n translator -v -f target/debug/libltranslator.so
# Move index.node to the right folder after generation
mv index.node src/nodejs

# Python bindings
echo "==> Python bindings"
# Native import, but requires renaming
cp target/debug/libltranslator.so src/python/ltranslator.so

echo "Done!"
