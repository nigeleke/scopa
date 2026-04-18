#!/bin/bash
sed -i 's/#base_path/base_path/g' Dioxus.toml
cargo install cargo-binstall
cargo binstall dioxus-cli
dx bundle --release --platform=web --debug-symbols=false
ls -al
mkdir -p _site/app
mv target/dx/*/release/web/public _site/app
