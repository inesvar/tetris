- press key s open settings
- add 180° key
- assets load bug when too low in filesystem
- re-implement clipboard copy/paste button using arboard (https://crates.io/crates/arboard/3.4.1) 

# build for windows

Add the target (you can also change this for whatever target you're cross compiling for):

        rustup target add x86_64-pc-windows-gnu

You can build your crate easily with:

        cargo build --release --target x86_64-pc-windows-gnu
