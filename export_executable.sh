#!/bin/bash
folderName="tetrisApp" 
path=~/"$folderName"

cargo build --release

mkdir "$path"
cp ./target/release/tetris "$path"