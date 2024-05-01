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
    // 2024-05-01
    // 10:55-11:00 (5min)
    // 13:47-14:18 (31min)
    // 36min
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    // 袋の中: A, B, C枚

    // 100枚持ってる。
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    dp[a][b][c] = 1.0;
    for ai in a..100 {
        for bi in b..100 {
            for ci in c..100 {
                dp[ai+1][bi][ci] += dp[ai][bi][ci] * ai as f64 / (ai+bi+ci) as f64 ; 
                dp[ai][bi+1][ci] += dp[ai][bi][ci] * bi as f64  / (ai+bi+ci) as f64 ; 
                dp[ai][bi][ci+1] += dp[ai][bi][ci] * ci as f64  / (ai+bi+ci) as f64 ; 
            }
        }
    }
    // println!("dp[99][99][99] = {}", dp[99][99][99]);
    let mut e = 0.0;
    for bi in b..100 {
        for ci in c..100 {
            let count = (100 - a + bi - b + ci - c) as f64;
            let prob = dp[100][bi][ci];
            let diff = prob * count;
            // println!("a=100, b = {bi}, c = {ci}, count = {count}, prob = {prob}, diff={diff}");
            e += diff;
        }
    }
    for ci in c..100 {
        for ai in a..100 {
            let count = (ai - a + 100 - b + ci - c) as f64;
            let prob = dp[ai][100][ci];
            let diff = prob * count;
            // println!("a={ai}, b = 100, c = {ci}, count = {count}, prob = {prob}, diff={diff}");
            e += dp[ai][100][ci] * (ai - a + 100 - b + ci - c) as f64;
        }
    }
    for ai in a..100 {
        for bi in b..100 {
            e += dp[ai][bi][100] * (ai - a + bi - b + 100 - c) as f64;
        }
    }
    println!("{}", e);


}