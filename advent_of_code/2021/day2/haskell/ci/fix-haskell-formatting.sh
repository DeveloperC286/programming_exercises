#!/usr/bin/env sh

set -o errexit
set -o xtrace

ormolu --mode inplace src/*.hs tests/*.hs
