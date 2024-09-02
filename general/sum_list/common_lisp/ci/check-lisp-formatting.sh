#!/usr/bin/env sh

set -o errexit
set -o xtrace

temp=$(mktemp)

for i in $(find "." -type f | grep -i "[.]lisp$"); do
	sbcl --noinform --eval "(pprint (read (open \"${i}\")))" --eval '(quit)' | awk '{print tolower($0)}' >"${temp}"
	cmp "${i}" "${temp}" || (echo "${i} needs formatted." && exit 1)
done
