#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::vec;
use superslice::*;

fn main() {

    // # https://atcoder.jp/contests/abc322/submissions/46132430
    // # Eの誰かの回答。デバッグ用に作った。
    input! {
        n: usize,
        k: usize,
        p: usize,
    }
    let mut c = vec![];
    let mut a = vec![];
    for _i in 0..n {
        input! {c_: usize, a_: [usize; k]}
        c.push(c_);
        a.push(a_);
    }
    let sz = 1 << (4 * k);
    let mut dp = vec![vec![!0; sz]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..sz {
            if dp[i][j] == !0 {
                continue;
            }
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            let mut nxt = 0;
            for m in 0..k {
                let mut dd = (j >> (4 * m)) & 15;
                dd = min(dd + a[i][m], p);
                nxt |= dd << (4 * m);
            }
            dp[i + 1][nxt] = dp[i + 1][nxt].min(dp[i][j] + c[i]);
        }
    }
    let mut s = 0;
    for i in 0..k {
        s |= p << (4 * i);
    }
    let ans = if dp[n][s] == !0 { -1 } else { dp[n][s] as i64 };
    println!("{ans}");
}
