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
    // 2024-11-20 12:34-12:45 (11min)
    // 2024-11-20 20:31-20:43 (12min)
    // 23min
    input! {
        n: usize,
        t: usize,
        a: [isize; n],
    }
    // 旅全体で、売買はT個以下
    // 青木のコストを求めよ。

    let mut higest = a[n-1];
    let mut profits = vec![0; n];
    let mut max_profit = 0;
    for i in (0..n).rev() {
        profits[i] = higest - a[i];
        max_profit = max(max_profit, profits[i]);
        higest = max(higest, a[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        if profits[i] == max_profit {
            ans += 1;
        }
    }
    println!("{}", ans);
}