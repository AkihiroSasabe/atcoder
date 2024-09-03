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
    input! {
        n: usize,
        a: [usize; n]
    }

    // dp[i][r] := i番目をr番目にたおす
    let mut dp = vec![vec![0; 2]; n];
    dp[0][1] = a[0];
    for i in 1..n {
        dp[i][0] = max(a[i] * 2 + dp[i-1][1], dp[i-1][0]);
        dp[i][1] = max(a[i] + dp[i-1][0], dp[i-1][1]);
        // println!("dp[{i}] = {:?}", dp[i]);
    }

    println!("{}", max(dp[n-1][0], dp[n-1][1]));


}