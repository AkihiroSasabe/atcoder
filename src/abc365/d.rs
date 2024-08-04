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
        let temp = if ss[i] == 'R' {
            0
        }
        else if ss[i] == 'S' {
            1
        }
        else {
            2
        };
        s.push(temp);
    }

    // dp
    // r: 0, s: 1, p: 2
    let mut dp = vec![vec![0; 3]; n];
    if s[0] == 0 {
        dp[0][2] = 1;
        dp[0][1] = -1000000000;
    }
    else if s[0] == 1{
        dp[0][0] = 1;
        dp[0][2] = -1000000000;
    }
    else if s[0] == 2 {
        dp[0][0] = -1000000000;
        dp[0][1] = 1;
    }

    for i in 1..n {
        if s[i] == 0 {
            dp[i][2] = max(dp[i-1][0] + 1, dp[i-1][1] + 1);
            dp[i][1] = -1000000000;

            dp[i][0] = max(dp[i-1][1], dp[i-1][2]);
        }
        else if s[i] == 1 {
            dp[i][0] = max(dp[i-1][1] + 1, dp[i-1][2] + 1);            
            dp[i][2] = -1000000000;

            dp[i][1] = max(dp[i-1][0], dp[i-1][2]);
        }
        else if s[i] == 2 {
            dp[i][0] = -1000000000;
            dp[i][1] = max(dp[i-1][0] + 1, dp[i-1][2] + 1);

            dp[i][2] = max(dp[i-1][0], dp[i-1][1]);
        }
    }
    let mut ans = 0;
    for i in 0..3 {
        ans = max(ans, dp[n-1][i]);
    }
    println!("{}", ans);


}