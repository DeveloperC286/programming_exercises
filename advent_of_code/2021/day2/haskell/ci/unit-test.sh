#!/usr/bin/env sh

set -o errexit
set -o xtrace

cabal test
