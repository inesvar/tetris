#!/bin/bash
folderName="tetrisWindowsApp" 
path=~/"$folderName"

# building the executable
cargo build --release --target x86_64-pc-windows-gnu

# copies the current executable and assets to "~/tetrisApp"
mkdir "$path"
cp ./target/x86_64-pc-windows-gnu/release/tetris.exe "$path"
cp -r ./src/assets "$path"