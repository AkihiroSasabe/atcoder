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
    // 2023-01-18 21:20-21:56 (36min)
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }
    let MODULO = 998244353;
    let mut dp = vec![vec![0; 3_001]; n];
    for i in 0..n {
        let mut cum = 0;
        for j in 0..3001 {
            if i == 0 {
                if a[i] <= j && j <= b[i] {
                    dp[i][j] = 1;
                }                
            }
            else {
                cum += dp[i-1][j];
                cum %= MODULO;
                if a[i] <= j && j <= b[i] {
                    dp[i][j] = cum;
                }
            }
        }
    }

    let mut ans = 0;
    for i in a[n-1]..b[n-1]+1 {
        ans += dp[n-1][i];
        ans %= MODULO;
    }
    println!("{}", ans);

}