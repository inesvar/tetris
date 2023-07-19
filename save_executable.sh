#!/bin/bash
folderName="tetrisApp"

path=~/"$folderName"
cargo build
mkdir "$path";
cp ./target/release/tetris "$path"
cp -r ./src/assets "$path"