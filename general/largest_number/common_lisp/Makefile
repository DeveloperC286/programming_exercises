DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: 
	echo "No default target."

# renovate: depName=clfoundation/sbcl
SBCL_VERSION=2.5.10@sha256:6dd10467895dd90c99d7a0cbef05d178d632811651ad39e168936bda3410da8e

.PHONY: check-common-lisp-formatting
check-common-lisp-formatting:
	docker run $(DOCKER_RUN_OPTS) clfoundation/sbcl:$(SBCL_VERSION) sbcl --noinform --load /workspace/largest_number.lisp --eval "(quit)"

.PHONY: fix-common-lisp-formatting
fix-common-lisp-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) clfoundation/sbcl:$(SBCL_VERSION) sbcl --noinform --load /workspace/largest_number.lisp --eval "(quit)" > /workspace/largest_number.lisp