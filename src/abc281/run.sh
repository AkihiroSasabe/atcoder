#!/bin/bash

# How to use:
#     $ bash run.sh <problem name>
# For example:
#     $ bash run.sh a
# Which is equal to
#     $ cargo run --bin abcXXX_a

CONTEST_NAME=$(echo `pwd` | rev | cut -f -1 -d "/" | rev )

echo $CONTEST_NAME
cargo run --bin ${CONTEST_NAME}_${1}
# cargo run --bin ${CONTEST_NAME}_${1} --release < e_testcase.txt