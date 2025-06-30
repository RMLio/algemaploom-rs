#!/bin/bash
#
# AlgeMapLoom JNI bindings native Rust library building script
# License: MIT (2024)
#
set -e

# Remove Rust library as resource
echo "==> Cleaning up old Rust library"
rm -rf src/main/resources/Linux/*
rm -rf src/main/resources/Windows/*
rm -rf src/main/resources/Apple-x86_64/*
rm -rf src/main/resources/Apple-aarch64/*
cd ../../../
rm -rf target

# Compile Rust library
echo "==> Building Rust library for Linux x86..."
echo "==> Check target/x86_64-unknown-linux-gnu/log.txt for errors!"
mkdir -p target/x86_64-unknown-linux-gnu
cargo build --lib --release --features=jni --target=x86_64-unknown-linux-gnu >target/x86_64-unknown-linux-gnu/log.txt 2>&1

echo "==> Building Rust library for Windows x86..."
echo "==> Check target/x86_64-pc-windows-gnu/log.txt for errors!"
mkdir -p target/x86_64-pc-windows-gnu
cargo build --lib --release --features=jni --target=x86_64-pc-windows-gnu >target/x86_64-pc-windows-gnu/log.txt 2>&1

if [[ "$OSTYPE" == "darwin"* ]]; then
	echo "==> Building Rust library for Apple x86..."
	mkdir -p target/x86_64-apple-darwin
	cargo build --lib --release --features=jni --target=x86_64-apple-darwin >target/x86_64-apple-darwin/log.txt 2>&1
	echo "==> Building Rust library for Apple aarch64..."
	mkdir -p target/aarch64-apple-darwin
	cargo build --lib --release --features=jni --target=aarch64-apple-darwin >target/aarch64-apple-darwin/log.txt 2>&1
else
	echo "==> Building Rust library for Apple x86/aarch64 skipped, only supported on native Apple due to license issues."
fi

# Copy compiled Rust library to Java Jar resource location
echo "==> Copying Rust library to Java Jar resource"
cp target/x86_64-unknown-linux-gnu/release/libltranslator.so src/java/algemaploom/src/main/resources/Linux/
cp target/x86_64-pc-windows-gnu/release/ltranslator.dll src/java/algemaploom/src/main/resources/Windows/
if [[ "$OSTYPE" == "darwin"* ]]; then
	cp target/x86_64-apple-darwin/release/libltranslator.dylib src/java/algemaploom/src/main/resources/Apple-x86_64
	cp target/aarch64-apple-darwin/release/libltranslator.dylib src/java/algemaploom/src/main/resources/Apple-aarch64
fi

echo "Done!"
