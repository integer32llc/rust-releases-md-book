#!/bin/bash

# Meant to be run manually by someone with permissions to the repo. maybe someday in GH actions

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cargo run
cd target/book
mdbook build
cd book

git init
git remote add upstream git@github.com:integer32llc/rust-releases-md-book.git
git fetch upstream

touch .
git add -A .
git commit -m "rebuild pages at ${rev}"
git push -f upstream HEAD:gh-pages
