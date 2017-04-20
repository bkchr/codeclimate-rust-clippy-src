#!/usr/bin/env bash
set -e

docker run -it -v $(pwd)/src:/src rust-clippy-build /bin/sh -c "cd codeclimate-clippy && cargo test"

curl -L https://github.com/codeclimate/codeclimate/archive/master.tar.gz | tar xvz
cd codeclimate-* && sudo make install && cd ..

docker build -t codeclimate/codeclimate-rust-clippy --rm=false .

cd tests
codeclimate analyze --dev | diff - expected_output.out
