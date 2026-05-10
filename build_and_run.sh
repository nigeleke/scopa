!#bash
cd dev
gleam run -m scripts/generate_version
cd ..

cd core
gleam build
gleam test
gleam run -m lustre/dev start
cd ..
