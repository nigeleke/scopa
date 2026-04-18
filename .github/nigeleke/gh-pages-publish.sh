#!/bin/bash
sed -i 's/#base_path/base_path/g' Dioxus.toml
cargo install cargo-binstall
cargo binstall dioxus-cli
dx bundle --release --platform=web --debug-symbols=false
mkdir -p site/app
mv target/dx/*/release/web/public site/app
ls -al site
ls -al site/app
