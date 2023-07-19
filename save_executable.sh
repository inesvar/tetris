#!/bin/bash
folderName="tetrisApp"

path=~/"$folderName"
cargo build
rm -rf "$path"
mkdir "$path";
cp ./target/debug/tetris "$path"
cp -r ./src/assets "$path"