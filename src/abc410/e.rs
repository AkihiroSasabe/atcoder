#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-14 21:21-21:48 (27min)
    input! {
        n: usize,
        h: usize, // 体力
        m: isize, // 魔料
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            bi: isize,
        }
        a.push(ai); // 魔法使わない.体力ai減らす
        b.push(bi); // 魔法使う.魔料biしょうひ 
    }

    let mut dp = vec![vec![-1; h+1]; n+1];

    dp[0][h] = m;
    for i in 0..n {
        for hh in 0..h+1 {
            // 魔法を使う
            if dp[i][hh] >= b[i] {
                dp[i+1][hh] = max(dp[i+1][hh], dp[i][hh] - b[i]);
            }

            // 魔法を使わない
            if hh >= a[i] {
                dp[i+1][hh-a[i]] = max(dp[i+1][hh-a[i]], dp[i][hh]);
            }
        }
    }

    let mut ans = 0;
    for i in 1..n+1 {
        for hh in 0..h+1 {
            if dp[i][hh] >= 0 {
                ans = max(ans, i);
            }
        }
    }
    println!("{}", ans);

}