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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize
    }
    let modulus: usize = 998_244_353;
    let mut dp = vec![vec![0; 11]; n];
    for i in 1..10 {
        dp[0][i] = 1;
    }
    for i in 1..n {
        for j in 1..10 {
            dp[i][j] = dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1];
            dp[i][j] %= modulus;
        }
    }
    let mut ans = 0;
    for i in 1..10 {
        ans += dp[n-1][i];
        ans %= modulus;
    }
    println!("{}", ans);


}