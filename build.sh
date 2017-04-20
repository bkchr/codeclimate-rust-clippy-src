#!/usr/bin/env bash
set -e

docker build -t rust-clippy-build -f Build.dockerfile .

docker run -it -v $(pwd)/src:/src rust-clippy-build /bin/sh -c "cd rust-clippy && cargo build --release && cd ../codeclimate-clippy && cargo build --release"

mkdir -p bin
cp src/rust-clippy/target/release/cargo-clippy bin/
cp src/codeclimate-clippy/target/release/codeclimate-clippy bin/

