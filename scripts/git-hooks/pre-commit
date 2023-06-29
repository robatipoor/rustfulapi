#!/bin/bash
"$(git rev-parse --git-dir)/hooks/pre-commit-cargo-fmt"
fmt=$?
"$(git rev-parse --git-dir)/hooks/pre-commit-typos"
typos=$?
if [[ $fmt -ne 0 ]] || [[ $typos -ne 0 ]];then
    exit 1
fi
