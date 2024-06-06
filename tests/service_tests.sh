#!/bin/bash

set -e

cd ../posts
go test -v ./... -tags=service_tests

cd ../stats
source ../.venv/bin/activate
python3 -m unittest service_tests.py -v
