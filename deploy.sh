#!/usr/bin/env bash

GIT_COMMIT_MSG_HEADER="New rust-clippy engine update"
GIT_COMMIT_MSG_CONTENT="Update based on commit https://github.com/bkchr/codeclimate-rust-clippy-src/commit/$(git rev-parse HEAD)"

git clone git@github.com:bkchr/codeclimate-rust-clippy.git codeclimate-rust-clippy
cd codeclimate-rust-clippy

mkdir -p bin

cp -f ../bin/cargo-clippy bin/
cp -f ../bin/codeclimate-clippy bin/

cp -f ../Dockerfile .
cp -f ../engine.json .
cp -f ../main.sh .
cp -f ../install-rust.sh .

if ! git diff --quiet
then
    git add .
    git commit -am "$GIT_COMMIT_MSG_HEADER" -m "$GIT_COMMIT_MSG_CONTENT"
    git push
fi

exit 0
