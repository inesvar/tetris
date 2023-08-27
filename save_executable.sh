#!/bin/bash
folderName="tetrisApp"

path=~/"$folderName"
cargo build
# erases the contents of ~/TetrisApp
rm -rf "$path"/tetris
# moves the executable to ~/TetrisApp
cp ./target/debug/tetris "$path"
# to test two players mode : ~/TetrisApp/tetris --two-remote
# then swap the IP adresses in the repo and execute with cargo run -- --two-remote