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
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
    }
    let MODULO: usize = 998244353;
    // let mut up: usize = 1;
    // let mut down: usize = 1;
    let mut dp = vec![vec![0; 2];n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        if a[i-1] != a[i] {
            dp[i][0] += dp[i-1][0];
            dp[i][0] %= MODULO;
        }
        if b[i-1] != a[i] {
            dp[i][0] += dp[i-1][1]; 
            dp[i][0] %= MODULO;
        }
        if a[i-1] != b[i] {
            dp[i][1] += dp[i-1][0]; 
            dp[i][1] %= MODULO;
        }
        if b[i-1] != b[i] {
            dp[i][1] += dp[i-1][1]; 
            dp[i][1] %= MODULO;
        }
    }
    println!("{}", (dp[n-1][0] + dp[n-1][1]) % MODULO);


}