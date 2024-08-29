#!/usr/bin/env sh

set -o errexit
set -o xtrace

ormolu --mode check src/*.hs tests/*.hs
