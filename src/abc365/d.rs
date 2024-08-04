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
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        ss: Chars // aoki
    }
    // 高橋 負けなし
    // t[i] != t[i+1]

    let mut s = vec![];
    for i in 0..n {
        let temp = match ss[i] {
            'R' => 0,
            'S' => 1,
            _ => 2,
        };
        s.push(temp);
    }

    // dp
    // r: 0, s: 1, p: 2
    let mut dp = vec![vec![0; 3]; n+1];
    let NINF = -1_000_000_000;
    for i in 0..n {
        let p0 = s[i];              // rock
        let p1 = (s[i] + 1) % 3;    // scissor
        let p2 = (s[i] + 2) % 3;    // paper

        dp[i+1][p0] = max(dp[i][p1], dp[i][p2]);
        dp[i+1][p1] = NINF;
        dp[i+1][p2] = max(dp[i][p0] + 1, dp[i][p1] + 1);
    }
    let mut ans = 0;
    for i in 0..3 {
        ans = max(ans, dp[n][i]);
    }
    println!("{}", ans);
}