#!/usr/bin/env bash

MODE="$1"

if [[ "release" = "$MODE" ]]; then
    cargo build --release
    cp ./target/release/saurus "$HOME/bin"
else
    cargo build 
    cp ./target/debug/saurus "$HOME/bin"
fi
