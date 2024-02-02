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

    // 2024-01-26 21:07-21:22 (15min)
    input! {
        n: usize,
        a: [usize; n]
    }
    let modulus = 998244353;

    // dp[i][x] := i回目のあとの左端の状態がxな数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;
    for i in 1..n {
        for x in 0..10 {
            let f = (x + a[i]) % 10;
            let g = (x * a[i]) % 10;
            dp[i][f] += dp[i-1][x];
            dp[i][g] += dp[i-1][x];
            dp[i][f] %= modulus;
            dp[i][g] %= modulus;
        }
    }
    for k in 0..10 {
        let ans = dp[n-1][k];
        println!("{}", ans);
    }


}