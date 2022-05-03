#!/bin/bash

set -eu

cd `dirname $0`

readonly EPO_BIN=${1:-../target/release/epo}

python arbitrary_tests.py --epo ${EPO_BIN} --repeat 1000
