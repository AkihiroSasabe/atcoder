#!/bin/bash

# How to use and:
#     $ code h.rs # (to create the correct code (naive solution or someone else's AC solution))
#     $ code make_testcase.py # (to create the random testcase)
#     $ bash debug.sh <problem name>
# For example:
#     $ bash debug.sh a
# Which is equal to
#     $ cargo run --bin abcXXX_a
#     $ cargo run --bin abcXXX_h

CONTEST_NAME=$(echo `pwd` | rev | cut -f -1 -d "/" | rev )

echo $CONTEST_NAME


loop_count=0
while true; do
    # ループ回数をインクリメント
    loop_count=$((loop_count + 1))
    echo "ループ回数: $loop_count"

    # (1)自動でランダムなテストケース作成
    python3 make_testcase.py > temp.txt
    # (2)自分の提出コードで解答
    my_output=$(cargo run --bin ${CONTEST_NAME}_${1} < temp.txt)
    # (3)誰かのACした提出コードで解答
    ac_output=$(cargo run --bin ${CONTEST_NAME}_h < temp.txt)

    echo ${my_output}
    echo ${ac_output}

    # (4)自分の解と、誰かのACコードの解が異なる値になった場合、ループを抜ける
    if [ "${my_output}" -ne "${ac_output}" ]; then
        break
    fi
done

echo "ループを抜けました。ループ回数: $loop_count"
