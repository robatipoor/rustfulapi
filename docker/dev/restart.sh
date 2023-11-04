#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

"$SCRIPT_DIR/down.sh"

"$SCRIPT_DIR/rm.sh"

"$SCRIPT_DIR/up.sh"

