#!/bin/sh

# Rust build
cargo build

# Java bindings
javac src/java/Translator.java

# NodeJS bindings
npm i
./node_modules/.bin/neon dist -n translator -v -f target/debug/libltranslator.so
mv index.node src/nodejs
