.PHONY: default
default: compile

.PHONY: check-haskell-formatting
check-haskell-formatting:
	ormolu --mode check lib/Day2.hs src/Main.hs tests/Tests.hs

.PHONY: fix-haskell-formatting
fix-haskell-formatting:
	ormolu --mode inplace lib/Day2.hs src/Main.hs tests/Tests.hs

.PHONY: check-haskell-linting
check-haskell-linting:
	hlint lib/Day2.hs src/Main.hs tests/Tests.hs

.PHONY: compile
compile:
	cabal build

.PHONY: unit-test
unit-test:
	cabal test
