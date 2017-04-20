#!/usr/bin/env bash
set -e

docker run -it -v $(pwd)/src:/src rust-clippy-build /bin/sh -c "cd codeclimate-clippy && cargo test"

git clone https://github.com/codeclimate/codeclimate.git codeclimate-cli-src
cd codeclimate-cli-src
git checkout 13bc36548f6713b73332d144a3470aa37fcf8e1
git apply ../codeclimate-engine.patch
sudo make install
cd ..

docker build -t codeclimate/codeclimate-rust-clippy --rm=false .

cd tests
codeclimate analyze --dev | diff - expected_output.out
