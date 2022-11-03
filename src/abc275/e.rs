#![allow(dead_code, unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let modulo = 998244353;
    let m_rev = mod_pow(m, modulo - 2, modulo);

    // println!("m_rev: {}", m_rev);

    // dp[ルーレット回転数kk][マスi]: kk回ルーレットを回したときに、マスiに居る確率
    let mut dp = vec![vec![0; n + 1]; k];
    for i in 1..(m + 1) {
        dp[0][i] += m_rev;
    }
    // println!("{:?}", dp[0]);

    for kk in 1..k {
        for i in 0..n {
            for j in 1..(m + 1) {
                let mut next = i + j;
                if next > n {
                    next = 2 * n - (i + j);
                }
                dp[kk][next] = (dp[kk][next] + (dp[kk - 1][i] * m_rev) % modulo) % modulo;
            }
        }
        // 以下のやり方だとACできない。(理由は分からない)
        // for i in 0..(n + 1) {
        //     // 左から来るケース
        //     for j in 1..(m + 1) {
        //         if i >= j {
        //             dp[kk][i] = (dp[kk][i] + (dp[kk - 1][i - j] * m_rev) % modulo) % modulo;
        //         }
        //     }
        //     // 右から来るケース
        //     // 次の位置 = N - (今の位置 + 出た目 - N)
        //     //          = 2N - 今の位置 - 出た目
        //     // 出た目 = 2N - 今の位置 - 次の位置 で、
        //     // 1 <= 出た目 <= m の制約がある
        //     for j in (i + 1)..n {
        //         if 2 * n <= m + i + j && 1 + i + j <= 2 * n {
        //             dp[kk][i] = (dp[kk][i] + (dp[kk - 1][j] * m_rev) % modulo) % modulo;
        //         }
        //     }
        // }
    }
    let mut ans = 0;
    for kk in 0..k {
        ans = (ans + dp[kk][n]) % modulo;
        // println!("{:?}", dp[kk]);
    }

    println!("{}", ans);
}

fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder;
}
