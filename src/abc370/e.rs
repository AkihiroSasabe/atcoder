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
use rand::Rng;
fn main() {
    // 2024-09-07 21:55-22:40 (45min) 
    // 2024-09-09 19:26-20:18 (52min, 降参, a[i]が正だと思ってた。負もありだとわからなくなった。)
    // 2024-09-09 20:18-21:00 (42min, 解説見た)
    // Total: 139min
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }
    let modulus: usize = 998244353;
    let mut cum = a.clone();
    for i in 1..n {
        cum[i] = cum[i] + cum[i-1];
    }
    cum.push(cum[n-1]);

    // dp[i] := i個目の前で切れる個数
    let mut dp = vec![0; n+1];
    dp[0] = 1; // ここは絶対切っていい

    let mut dp_cum = dp.clone();
    let mut  btree = BTreeMap::new();
    btree.insert(0, dp[0]);

    for i in 0..n {
        // println!("***** i = {:?}", i);
        // println!("btree = {:?}", btree);
        dp[i+1] = dp_cum[i];

        let cc = cum[i];

        if let Some(&val) = btree.get(&(cc - k)) {
            dp[i+1] += modulus - (val % modulus);
            dp[i+1] %= modulus;
        }

        *btree.entry(cum[i]).or_insert(0) += dp[i+1];
        dp_cum[i+1] = dp_cum[i] + dp[i+1];
        dp_cum[i+1] %= modulus;
        // println!("dp[{}] = {:?}", i+1, dp[i+1]);
    }
    // println!("dp = {:?}", dp);
    println!("{}", dp[n]);

}
