# Tetris in Rust

Currently our Tetris looks something like this.

![](tetris.png)

But it will get a lot, lot better.

## What fun can be had as of now

Currently you can play tetris alone locally just with :
```bash
cargo run
```

I's also possible to play tetris remotely by setting the SERVER_IP and VIEWER_IP in ./src/settings.rs to yours and your friend's IP and then running :
```bash
cargo run -- --two-remote
```

It doesn't work if the IPs can't ping each other, a solution to this is connecting to the same mobile hotspot.

## Now I wonder, how could this possibly get better...?

Well the menu, the buttons and all the parameters will get our attention soon. And then, maybe, we'll think about making a local two-player version.

## I can't have any fun because wtf is cargo doesn't work on my machine

Then you can refer to the Great Rust Documentation : https://doc.rust-lang.org/cargo/getting-started/installation.html.