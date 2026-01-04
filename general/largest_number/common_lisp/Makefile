.PHONY: default
default:
	echo "No default target."

.PHONY: check-common-lisp-formatting
check-common-lisp-formatting:
	sbcl --noinform --load largest_number.lisp --eval "(quit)"

.PHONY: fix-common-lisp-formatting
fix-common-lisp-formatting:
	sbcl --noinform --load largest_number.lisp --eval "(quit)" > largest_number.lisp
