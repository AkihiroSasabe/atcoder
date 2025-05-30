#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-24 10:05-
    input! {
        n: usize,
        a: [Usize1; n],
    }

    // 2^20 = 1_048_576
    solve_tle(n, a);
    // solve(n, a);
}



fn solve(n: usize, a: Vec<usize>) {
    // 2024-11-24 13:16-
    // dp[S] = 最小index ??

    let mut sets = vec![BTreeSet::new(); 20];
    for i in 0..n {
        sets[a[i]].insert(i);
    }

    // dp[mask] = i := maskを満たす集合が、i より左にある。
    let mut dp = vec![n+1; 1<<20];
    dp[0] = 0;
    let mut ans = 0;
    for mask in 0..1<<20 {
        // ここより右のindexから探す
        let index = dp[mask];

        for ch in 0..20 {
            let next_mask = ch | mask;
            if (mask & (1 << ch)) != 0 {
                // すでにある場合
                dp[next_mask] = min(dp[next_mask], dp[mask]);
            }
            else {
                // 二分探索か?
                if let Some(&first) = sets[ch].range(index..).next() {
                    if let Some(&second) = sets[ch].range(first+1..).next() {
                        dp[next_mask] = min(dp[next_mask], second + 1);
                    }
                }
            }
        }
        if dp[mask] != n + 1 {
            ans = max(ans, mask.count_ones() * 2);
        } 
    }
    println!("{}", ans);
}

fn solve_tle(n: usize, a: Vec<usize>) {
    let mut pres = vec![n; 20];
    let mut edges = vec![n; n];
    for i in 0..n {
        if pres[a[i]] != n {
            edges[i] = pres[a[i]];
        }
        pres[a[i]] = i;
    }

    let mut btree = BTreeSet::new();
    btree.insert(0);
    let mut dp: Vec<BTreeSet<usize>> = vec![btree; n];

    for i in 1..n {
        let ei = edges[i];
        dp[i] = dp[i-1].clone();
        if ei == n {
            continue
        }
        let mut pre_dp_ei = dp[ei].clone();
        for mask in pre_dp_ei {
            dp[i].insert(mask | (1 << a[i]));
        }
    }

    // // Debug
    // for i in 0..n {
    //     print!("dp[{i}] = ");
    //     for x in dp[i].iter() {
    //         print!("{:0b} ", x);
    //     }
    //     println!("");
    // }

    let mut ans = 0;
    for &mask in dp[n-1].iter() {
        let pop_count = mask.count_ones();
        ans = max(ans, 2 * pop_count);
    }
    println!("{}", ans);
}