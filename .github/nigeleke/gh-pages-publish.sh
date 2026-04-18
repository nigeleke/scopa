sed -i 's/#base_path/base_path/g' Dioxus.toml
dx bundle --release --platform=web --debug-symbols=false
git checkout Dioxus.toml
git status
mv target/dx/*/release/web/public _site/app
