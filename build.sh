#!/bin/bash -

# target host
cargo build --release

# target windows
cargo build --release --target x86_64-pc-windows-gnu
