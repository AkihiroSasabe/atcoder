#!/bin/bash

# How to use:
#     $ bash run.sh <problem name>
# For example:
#     $ bash run.sh a
# Which is equal to
#     $ cargo run --bin abcXXX_a

CONTEST_NAME=$(echo `pwd` | rev | cut -f -1 -d "/" | rev )

echo $CONTEST_NAME


loop_count=0
while true; do
    # ループ回数をインクリメント
    loop_count=$((loop_count + 1))
    echo "ループ回数: $loop_count"


    # (1)自動でランダムなテストケース作成
    python3 make_testcase_e.py > temp.txt
    # (2)自分の提出コードで解答
    my_output=$(cargo run --bin ${CONTEST_NAME}_e < temp.txt)
    # (3)誰かのACした提出コードで解答
    # https://atcoder.jp/contests/abc322/submissions/46132430
    ac_output=$(cargo run --bin ${CONTEST_NAME}_f < temp.txt)

    echo ${my_output}
    echo ${ac_output}

    # 自分の解と、誰かのACコードの解が異なる値になった場合、ループを抜ける
    if [ "${my_output}" -ne "${ac_output}" ]; then
        break
    fi

done

echo "ループを抜けました。ループ回数: $loop_count"
