DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: 
	echo "No default target."

# renovate: depName=clfoundation/sbcl
SBCL_VERSION=2.2.4@sha256:92b104e15870c8d9cd1b0b075e9f9143f10a1c8e4375032fceba291aca2fa0df

.PHONY: check-common-lisp-formatting
check-common-lisp-formatting:
	docker run $(DOCKER_RUN_OPTS) clfoundation/sbcl:$(SBCL_VERSION) sbcl --noinform --load /workspace/sum_list.lisp --eval "(quit)"

.PHONY: fix-common-lisp-formatting
fix-common-lisp-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) clfoundation/sbcl:$(SBCL_VERSION) sbcl --noinform --load /workspace/sum_list.lisp --eval "(quit)" > /workspace/sum_list.lisp