#!/bin/bash
folderName="tetrisWindowsApp" 
path=~/"$folderName"

cargo build --release --target x86_64-pc-windows-gnu

mkdir "$path"
cp ./target/x86_64-pc-windows-gnu/release/tetris.exe "$path"
cp -r ./src/assets "$path"