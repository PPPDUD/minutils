#!/usr/bin/bash
mkdir -p dist/linux
cargo build --release
rm target/x86_64-unknown-linux-musl/release/*.*
mv target/x86_64-unknown-linux-musl/release/minutils-* dist/linux
mv target/x86_64-unknown-linux-musl/release/minutils dist/linux
rm -f dist/linux/minutils.deb
fpm \
	-s dir -t deb \
	-p dist/linux/minutils.deb \
	--name minutils \
	--license MIT \
	--version 0.1.0 \
	--architecture amd64 \
	--description "Minimal implementations of Unix-like utilities in Rust." \
	--url "https://github.com/PPPDUD/minutils" \
	--maintainer "PPPDUD <mojavesoft@gmail.com>" \
	dist/linux/=/usr/bin/ \