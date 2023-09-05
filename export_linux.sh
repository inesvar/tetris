#!/bin/bash
folderName="tetrisApp" 
path=~/"$folderName"

# building the executable
cargo build --release

# copies the current executable and assets to "~/tetrisApp"
mkdir "$path"
cp ./target/release/tetris "$path"
cp -r ./src/assets "$path"