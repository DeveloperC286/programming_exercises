#!/usr/bin/env sh

set -o errexit
set -o xtrace

hlint src/*.hs tests/*.hs
