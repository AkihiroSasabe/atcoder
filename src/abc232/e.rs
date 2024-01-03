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
    // 2024-01-03 12:34-13:04 (30min)
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    }
    let modulus: usize = 998244353;

    // 思いついた
    let mut dp = vec![vec![0; 4]; k+1];

    // 0: ゴールと距離0
    // 1: ゴールと距離1で、同じ行にいる
    // 2: ゴールと距離1で、同じ列にいる
    // 3: ゴールと距離2
    if x1 == x2 && y1 == y2 {
        dp[0][0] = 1;
    }
    else if x1 == x2 && y1 != y2 {
        // 同じ行
        dp[0][1] = 1;
    }
    else if x1 != x2 && y1 == y2 {
        // 同じ列
        dp[0][2] = 1;
    }
    else if x1 != x2 && y1 != y2 {
        dp[0][3] = 1;
    }

    for i in 0..k {
        dp[i+1][0] = dp[i][1] + dp[i][2];
        dp[i+1][1] = dp[i][0] * (w - 1) % modulus + dp[i][1] * (w - 2) % modulus + dp[i][3] % modulus;
        dp[i+1][2] = dp[i][0] * (h - 1) % modulus + dp[i][2] * (h - 2) % modulus + dp[i][3] % modulus;
        dp[i+1][3] = dp[i][1] * (h - 1) % modulus + dp[i][2] * (w - 1) % modulus + dp[i][3] * (h + w - 1 - 3) % modulus;
        
        dp[i+1][0] %= modulus;
        dp[i+1][1] %= modulus;
        dp[i+1][2] %= modulus;
        dp[i+1][3] %= modulus;
    }
    // println!("dp = {:?}", dp);

    println!("{}", dp[k][0]);

}