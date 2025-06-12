# So new files are owned by the user.
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: check-common-lisp-formatting fix-common-lisp-formatting check-shell-formatting fix-shell-formatting

check-common-lisp-formatting:
	docker pull clfoundation/sbcl:2.2.4
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) clfoundation/sbcl:2.2.4 sbcl --noinform --load /workspace/sum_list.lisp --eval "(quit)"

fix-common-lisp-formatting:
	docker pull  clfoundation/sbcl:2.2.4
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) clfoundation/sbcl:2.2.4 sbcl --noinform --load /workspace/sum_list.lisp --eval "(quit)" > /workspace/sum_list.lisp