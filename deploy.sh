#!/usr/bin/env bash
set -e

GIT_COMMIT_MSG="New rust-clippy engine update\n\nUpdate based on commit https://github.com/bkchr/codeclimate-rust-clippy-src/commit/$(git rev-parse HEAD)"

git clone git@github.com:bkchr/codeclimate-rust-clippy.git codeclimate-rust-clippy
cd codeclimate-rust-clippy

mkdir -p bin

cp -f ../bin/cargo-clippy bin/
cp -f ../bin/codeclimate-clippy bin/

cp -f ../Dockerfile .
cp -f ../engine.json .
cp -f ../main.sh .
cp -f ../install-rust.sh .

git commit -am "$GIT_COMMIT_MSG"
git push
