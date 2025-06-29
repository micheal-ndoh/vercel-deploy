#!/bin/bash
set -e

# Install the musl target
rustup target add x86_64-unknown-linux-musl


cargo build --bin handler --release --target x86_64-unknown-linux-musl