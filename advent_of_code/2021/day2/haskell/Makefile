# So new files are owned by the user.
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: check-haskell-formatting check-shell-formatting fix-haskell-formatting fix-shell-formatting check-haskell-linting compile unit-test

check-haskell-formatting:
	docker build -t check-haskell-formatting -f ci/check-haskell-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace check-haskell-formatting

check-shell-formatting:
	docker pull mvdan/shfmt:v3.11.0-alpine
	docker run --rm -v $(PWD):/workspace -w /workspace mvdan/shfmt:v3.11.0-alpine --simplify --diff ci/* 

fix-haskell-formatting:
	docker build -t fix-haskell-formatting -f ci/fix-haskell-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) fix-haskell-formatting

fix-shell-formatting:
	docker pull mvdan/shfmt:v3.11.0-alpine
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:v3.11.0-alpine --simplify --write ci/* 

check-haskell-linting:
	docker build -t check-haskell-linting -f ci/check-haskell-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace check-haskell-linting

compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) compile

unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run --rm -v $(PWD):/workspace unit-test