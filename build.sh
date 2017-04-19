#!/usr/bin/env bash

docker build -trust-clippy-build -f Build.dockerfile .

docker run -it -v $(pwd)/src:/src rust-clippy-build /bin/sh -c "cd rust-clippy && cargo build --release && cd ../codeclimate-clippy && cargo build --release"

cp src/rust-clippy/target/release/rust-clippy bin/
cp src/codeclimate-clippy/target/release/codeclimate-clippy bin/

docker rm $(docker ps -a -q --filter=ancestor=rust-clippy-build)
docker rmi rust-clippy-build
