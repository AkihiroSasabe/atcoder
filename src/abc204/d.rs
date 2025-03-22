#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let t_max: usize = *t.iter().max().unwrap();
    let sum: usize = t.iter().sum();
    // dp[i][0] := i 番目までの料理を、全部調理し、かつ i番目をオーブン0で料理する。
    // let mut dp = vec![vec![vec![false; t_max + 2]; t_max + 2]; n];
    // dp[0][t[0]][0] = true;
    let mut dp = vec![vec![false; t_max * (n + 2) + 1]; n];
    dp[0][t[0]] = true;
    for i in 1..n {
        for diff in 0..t_max*n+1 {
            if dp[i-1][diff] {
                dp[i][diff + t[i]] = true;
                if diff >= t[i] {
                    dp[i][diff - t[i]] = true;
                }
                if t[i] >= diff {
                    dp[i][t[i] - diff] = true;
                }
            }
        }
    }
    for diff in 0..t_max*n+1 {
        if dp[n-1][diff] {
            let x = (sum + diff) / 2;
            let y = (sum - diff) / 2;
            println!("{}", max(x, y));
            return;
        }
    }



}