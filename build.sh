#!/bin/bash -

# target host
cargo build --release

# target windows
# assuming one run the environment from Unix and targetting Windows
# https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows
cargo build --release --target x86_64-pc-windows-gnu
