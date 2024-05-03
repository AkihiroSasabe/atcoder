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
    // 2024-05-03 18:11-18:22 (11min)
    input! {
        h: usize,
        n: usize,
    }
    let mut ab = vec![];
    for i in 0..n {
        input!{
            ai: usize, // 体力減らす
            bi: usize, // 消費
        }
        ab.push([ai, bi]);
    }
    ab.sort();

    // 魔力の最小を答える。
    let INF = 1 << 60;
    // dp[h] := 体力hの敵を倒すのに、必要な魔力
    let mut dp = vec![INF; h+1];
    dp[0] = 0;
    // 何回でも使えるDP
    for i in 0..n {
        for hj in 0..h+1 {
            dp[min(h, hj+ab[i][0])] = min(dp[min(h, hj+ab[i][0])], dp[hj] + ab[i][1]);
        }
    }
    println!("{}", dp[h]);
    

}