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
    // 2024-08-13 22:50-21:00 (10min)
    // 2024-08-14 9:27-9:48 (21min, 降参)
    // 2024-08-14 9:48-10:57 (1h9min)
    // 1h40min
    input! {
        n: usize,
        k: usize,
    }
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];
    let mut ab = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
        ab.push(((ai as f64 - 1.0) / bi as f64, i));
    }
    // ab.sort();
    ab.sort_by(|(x0, ind0), (x1, ind1)| x0.partial_cmp(x1).unwrap());
    // println!("ab = {:?}", ab);

    // ソートしてDP
    // dp[i][j] := i番目まででj個選んだときの最大値
    let mut dp: Vec<Vec<usize>> = vec![vec![1; k+1];n+1];
    for i in 0..n {
        let ind = ab[i].1;
        let ai = a[ind];
        let bi = b[ind];
        // println!("ai, bi = {}, {:?}", ai, bi);
        
        for j in 1..min(k+1, i+2) {
            let cand = ai * dp[i][j-1] + bi;
            dp[i+1][j] = max(dp[i+1][j], cand);

            dp[i+1][j] = max(dp[i+1][j], dp[i][j]);
        }
        // println!("dp[{}] = {:?}", i+1, dp[i+1]);
    }
    let ans = dp[n][k];
    println!("{}", ans);

}