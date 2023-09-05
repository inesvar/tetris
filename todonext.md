# TO DO SOON

- clean the code
- document it

# test on two computers 

- replace all the 127.0.0.1 by the commented line above

# LONG-TERM
- sending garbage after T-spin

# build for windows

Add the target (you can also change this for whatever target you're cross compiling for):

        rustup target add x86_64-pc-windows-gnu

You can build your crate easily with:

        cargo build --release --target x86_64-pc-windows-gnu
