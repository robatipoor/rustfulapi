#!/bin/bash

typos
result=$?

if [[ ${result} -ne 0 ]] ; then
    cat <<\EOF
There are some typo issues, run `typos --write-changes && git add .` first.
EOF
    exit 1
fi