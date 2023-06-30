#!/bin/bash

cargo fmt -- --check
result=$?

if [[ ${result} -ne 0 ]] ; then
    cat <<\EOF
There are some code style issues, run `cargo fmt && git add .` first.
EOF
    exit 1
fi