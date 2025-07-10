DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: compile

.PHONY: check-haskell-formatting
check-haskell-formatting:
	docker build -t check-haskell-formatting -f ci/check-haskell-formatting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-haskell-formatting

# renovate: depName=mvdan/shfmt
SHFMT_VERSION=v3.12.0-alpine@sha256:204a4d2d876123342ad394bd9a28fb91e165abc03868790d4b39cfa73233f150

.PHONY: check-shell-formatting
check-shell-formatting:
	docker run $(DOCKER_RUN_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --diff ci/* 

.PHONY: fix-haskell-formatting
fix-haskell-formatting:
	docker build -t fix-haskell-formatting -f ci/fix-haskell-formatting.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) fix-haskell-formatting

.PHONY: fix-shell-formatting
fix-shell-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --write ci/* 

.PHONY: check-haskell-linting
check-haskell-linting:
	docker build -t check-haskell-linting -f ci/check-haskell-linting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-haskell-linting

.PHONY: compile
compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) compile

.PHONY: unit-test
unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) unit-test