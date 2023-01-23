#!/usr/bin/env bash

cargo build
cp ./target/debug/saurus "$HOME/bin"
