# Tetris in Rust

Currently our Tetris looks something like this. 

![](tetris.png)

But it will get a lot better !

## What sort of Tetris is this ?

Our implementation satisfies the *Super Rotation System*, that means the starting positions, the rotations and wall-kicks are conforming to this standard.

About the fall speed, it's not standard, because we haven't found exact specifications.

## What fun can be had as of now

Currently you can play tetris alone locally just with :
```bash
cargo run
```

It works well, only it's more fun to play with others right ?

I's also possible to play tetris remotely by setting the SERVER_IP and VIEWER_IP in ./src/settings.rs to yours and your friend's IP and then running :
```bash
cargo run -- --two-remote
```

It doesn't work if the IPs can't ping each other, a solution to this is connecting to the same mobile hotspot.

The implementation of the two-player mode is incomplete. Currently it works exactly as if you were both playing alone but you could see each others screen.

## Now I wonder, how could this possibly get better...?

Well the menu, the buttons and all the parameters are currently getting our attention so there will be a great UI soon. And then, we'll think about making synchronization mecanisms so two players can actually play together remotely, as in, the same pieces are falling, the game stops when somebody loses, etc. We're getting there !

In the long run, it would be nice two have a local two player version.

## I can't have any fun because cargo doesn't work on my machine :/

Then you can refer to the Great Rust Documentation : https://doc.rust-lang.org/cargo/getting-started/installation.html.


## You're somewhat familiar with Rust and also somehow distressed by the hazardous speed at which pieces are falling?

Then you're interested by the variable *gravity* in main.rs, set it how it fits you.