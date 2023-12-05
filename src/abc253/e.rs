use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
fn main() {
    // 2023-12-05 16:18-16:41 (WA: 23min)
    // 2023-12-05 16:41-17:12 (Debug: 31min, 自力でdebugはできなった。atcoder companionを使ってしまった。)
    // total 54min
    input! {
        n: usize,
        m: usize,
        k: isize,
    }
    const MODULUS: isize = 998_244_353;

    // dp[現在の項数][値] := ありえる数
    let mut dp = vec![vec![0; m+1]; n];

    let mut cum = vec![0; m+1];
    for i in 1..m+1 {
        dp[0][i] = 1;
        cum[i] = cum[i-1] + dp[0][i];
    }

    for i in 1..n {
        for j in 1..m+1 {
            // 左から足す
            dp[i][j] += cum[max(0, j as isize - k) as usize];
            dp[i][j] %= MODULUS;

            // 右から足す
            if k != 0 {
                dp[i][j] += cum[m] - cum[min(m, j + k as usize - 1)];
            }
            else {
                // k == 0 のときだけ、重複してカウントしてしまうので注意がいる
                dp[i][j] += cum[m] - cum[min(m, j)];
            }
            dp[i][j] %= MODULUS;
            dp[i][j] += MODULUS;
            dp[i][j] %= MODULUS;
        }
        cum[0] = 0;
        for j in 1..m+1 {
            cum[j] = cum[j-1] + dp[i][j];
            cum[j] %= MODULUS;
        }
    }
    let mut ans = 0;
    for i in 1..m+1 {
        ans += dp[n-1][i];
        ans %= MODULUS;
    }

    println!("{}", ans);


}