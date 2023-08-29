#!/bin/bash
folderName="tetrisApp" 
path=~/"$folderName"

# building the executable
cargo build

# copies the current executable and assets to "~/tetrisApp"
mkdir "$path"
cp ./target/debug/tetris "$path"
cp -r ./src/assets "$path"

#launches one player in the current directory
cargo run

# the idea is then to execute tetris in ~/tetrisApp in another terminal 
# so that the prints don't get mixed
#
# cd ~/tetrisApp
# ./tetris