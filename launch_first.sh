#!/bin/bash
folderName="tetrisApp" 
path=~/"$folderName"

# building the executable
cargo build --release

# copies the current executable and assets to "~/tetrisApp"
mkdir "$path"
cp ./target/release/tetris "$path"
cp -r ./src/assets "$path"

#launches one player in the current directory
cargo run --release

# the idea is then to execute tetris in ~/tetrisApp in another terminal 
# so that the prints don't get mixed
#
# cd ~/tetrisApp
# ./tetris