#!/bin/bash
echo "==== Building With Default Features ===="
cargo run
echo "==== Building With Libcore ===="
cargo run --features libcore