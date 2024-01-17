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
use std::collections::{HashSet, BTreeSet};
use std::usize;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // println!("false = {:?}", false as usize); // 0
    // println!("true  = {:?}", true as usize); // 1

    // https://algo-logic.info/digit-dp/

    // 2024-01-15 19:50-21:00 (1h10m)
    // 2024-01-16 19:14-21:57 (2h43min)
    //                     21:09- 21:57nyaanの解説ac
    // total: 3h53min
    input! {
        n: Chars
    }

    // char -> usize に変換
    let mut nn = vec![];
    for i in 0..n.len() {
        nn.push(cast(n[i]));
    }
    let max_sum = 9 * 14; // 126

    // ■DPの定義
    // dp[d][i][j][f] := 個数
    // d := 先頭からd桁目まで確定
    // i := 桁和
    // j := % sの 値 (余り)
    // f := 0なら先頭からd桁目まではN未満、1ならNと一致
    // ■遷移
    // dp[d+1][i+t][(j *10+t)%s][f && N[d] - '0' == t] += dp[d][i][j][f]
    // ■初期化の意味
    // dp[0][0][0][1] = 1
    // N= 123を、
    // Na=0123と考えておく。
    // dp[0][0][0][1]=1
    // 先頭から0桁目 (この場合、Na[0]==0で、桁和は0になり、0をsで割った余りは0で、Na[0]と一致しているので、1個
    
    // 21:09- 21:57nyaanの解説ac
    // 計算量: 126 * 14 * 127 * 
    let mut ans = 0;
    // dp[m][s][r][f]
    for d in 1..max_sum+1 {
        // d := 約数 (=桁和)
        let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 2]; max_sum]; max_sum + 1]; n.len()+1];
        dp[0][0][0][1] = 1;
        for m in 0..n.len() {
            // m := 確定させる桁数

            for s in 0..max_sum+1 {
                // s: 桁和 (dは約数も兼ねているので注意)

                for r in 0..max_sum {
                    // r := m で割った余り

                    for x in 0..10 {
                        // x := m桁目の数字

                        for f in 0..2 {
                            // f: 状態 (0のときN以下、1のときNと一致)

                            let s2 = s + x;
                            // 総和が、max_sumを超えたらスキップ
                            if s2 > max_sum {continue}
                            let r2 = (r * 10 + x) % d;
                            
                            // Nを超えるとき (途中まで一致していて、注目する桁mの数字がN[m]を超えるとき)
                            if f == 1 && nn[m] < x {continue}
                            let f2 = f & ((nn[m] == x) as usize);

                            // 遷移
                            dp[m+1][s2][r2][f2] += dp[m][s][r][f];
                        }
                    }
                }
            }
        }
        ans += dp[n.len()][d][0][0] + dp[n.len()][d][0][1];
    }
    println!("{}", ans);

    // Debug
    // for i in 1..n {
    //     let (s, is_good) = judge_good(i);
    //     println!("i = {:04}, s = {:04}, good = {}", i, s, is_good)
    // }

    // i:       a(i),         r(i) := a(i) % s
    // i+1:    a(i+1)      r(i+1) := a(i+1) % s
    // a(i+1) = ai + x(i+1) * pow10
    // r(i+1) = ri + (x(i+1) * pow10) % s

    // 条件1: x % s == 0    <=第i桁目までの剰余がr (mod s)なら、第i+1桁目は、
    // 条件2: 桁和がs       <=簡単
    // 第i桁目までの剰余がr
}

fn cast(x: char) -> usize {
    x as usize - '0' as usize
}

fn judge_good(x: usize) -> (usize, bool) {
    let mut x_temp = x;
    let mut r = 0;
    while x_temp != 0 {
        r += x_temp % 10;
        x_temp /= 10;
    }
    return (r, x % r == 0)
}