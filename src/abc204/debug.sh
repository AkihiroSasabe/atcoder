#!/bin/bash

# How to use:
#     (1) Copy the following 3 scripts to target contest directry.
#     $ cp make_testcase.py solver.py debug.sh ./abcXXX/
#     $ cp ../make_testcase.py ../solver.py ../debug.sh ./
#     (2) Create the correct Python or Rust script (naive solution or someone else's AC solution)
#     $ code solver.py 
#     $ code h.rs 
#     (3) Create the random test case
#     $ code make_testcase.py 
#     (4) Run the debugger as follows. The testcases is generated unitil wrong_solver and correct_solver submit diffrent outputs.
#     $ bash debug.sh <problem name>
# For example:
#     $ bash debug.sh a
# Which is equal to running the two following scripts: 
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
    python3 make_testcase.py > test_case.txt
    # (2)自分の提出コードで解答
    my_output=$(cargo run --bin ${CONTEST_NAME}_${1} < test_case.txt)
    # (3)誰かのACした提出コードで解答
    # Rust h問題を使うこと
    ac_output=$(cargo run --bin ${CONTEST_NAME}_h < test_case.txt)
    # Python
    # ac_output=$(python3 solver.py < test_case.txt)

    echo "My answer ===="
    echo ${my_output}
    echo ${my_output} > output_wa.txt
    echo "Correct answer ===="
    echo ${ac_output}
    echo ${ac_output} > output_ac.txt

    # (4)自分の解と、誰かのACコードの解が異なる値になった場合、ループを抜ける
    # if [ "${my_output}" -ne "${ac_output}" ]; then
    if [ "${my_output}" == "" ]; then
        echo Runtime error occured in my_output. 
        break
    fi
    if [ "${ac_output}" == "" ]; then
        echo Runtime error occured in ac_output. 
        break
    fi
    if [ "${my_output}" != "${ac_output}" ]; then
        break
    fi
done

echo "ループを抜けました。ループ回数: $loop_count"
