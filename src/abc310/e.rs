#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    // 考察
    // f(i, j) := match {
    //     i == j => Ai,
    //     i < j  => f(i, j-1) ￢∧ Aj,
    //     i > j  => 0
    // }
    // f(i, j)の表 (i > j は定義不可なので"-"にしているが、後で和を取るので実質0と考えて良い)
    // 入力1の例
    //         j:  0   1   2   3   4
    //      s[j]:  0   0   1   1   0
    // ==============================
    // f(0,j):     0   1   0   1   1
    // f(1,j):     -   1   0   1   1
    // f(2,j):     -   -   0   1   1
    // f(3,j):     -   -   -   1   1
    // f(4,j):     -   -   -   -   0 
    // 求める答えは、j=0, 1, 2, ..., n について
    // f(i, j) が 1 になる個数 state_num[j][1] の総和である。

    let mut ans = 0;

    // 0の個数, 1の個数を数えていく
     // state_num[j][0] := 上の表でj列の0の個数の和
     // state_num[j][1] := 上の表でj列の1の個数の和
    let mut state_num = vec![vec![0, 0]; n];

    let mut num_0 = (s[0] as usize - '0' as usize) ^ 1; // 先頭文字が0なら1個, 1なら0個にしたいので1でXORを取っている
    let mut num_1 = s[0] as usize - '0' as usize; // 先頭文字が0なら0個, 1なら1個にしたい
    state_num[0][0] = num_0;
    state_num[0][1] = num_1;
    
    let mut ans = state_num[0][1];
    for j in 1..n {
        if s[j] == '0' {
            // 0になる状態は、i == j のときだけなので1個
            state_num[j][0] += 1;
            // i != j については、NAND(否定論理積)が全部1になるので、1の状態数は前回の状態数の総和に等しい
            state_num[j][1] += state_num[j-1][0] + state_num[j-1][1];
        }
        else if s[j] == '1' {
            // 0になる状態数は、前回の状態が1のときだけ
            state_num[j][0] += state_num[j-1][1];
            // 1になる状態数は、前回の状態が0のときと、i==jのとき
            state_num[j][1] += state_num[j-1][0] + 1;
        }
        ans += state_num[j][1];
    }

    println!("{}", ans);
}