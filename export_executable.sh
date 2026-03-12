#!/bin/bash
folderName="tetrisApp" 
path=~/"$folderName"

cargo build --release

mkdir -p "$path"
cp ./target/release/tetris "$path"
cp -r ./assets "$path"