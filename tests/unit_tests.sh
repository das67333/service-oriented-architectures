#!/bin/bash

set -e

cd ../auth
cargo test

cd ../posts
go test -v ./...

cd ../stats
source ../.venv/bin/activate
python3 -m unittest unit_tests.py -v
