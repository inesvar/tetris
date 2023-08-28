#!/bin/bash
folderName="tetrisApp" 
#
# this folder has to contain :
#   - the assets folder
#   - a file named "remote_ip.txt" with the same content than "local_ip.txt" from this repo
#   - a file named "local_ip.txt" with the same content than "remote_ip.txt" from this repo

path=~/"$folderName"
cargo build
# erases the executable
rm -rf "$path"/tetris
# copies the current executable
cp ./target/debug/tetris "$path"

#launch one player
cargo run

# the idea is then to execute tetris in tetrisApp in another terminal 
# so that the prints don't get mixed