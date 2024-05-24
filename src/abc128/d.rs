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
    // 2024-05-24 18:38-19:45 (1h7m)
    input! {
        n: usize,
        k: usize,
        v: [isize; n],
    }

    // 19:11

    // 順位がわかればいい。

    // dp_left[左から何番目?][捨てる個数] := 宝石の価値の最大値
    let mut dp_left = vec![vec![0; n+1]; n+1];
    dp_left[0][0] = 0;
    let mut set = BTreeSet::new();
    for i in 1..n+1 {
        set.insert(v[i-1]);
        dp_left[i][0] = dp_left[i-1][0] + v[i-1];
        for (ind, reduction) in set.iter().enumerate() {
            dp_left[i][ind + 1] = dp_left[i][ind] - reduction;
        }
    }

    // dp_right[右から何番目?][捨てる個数] := 宝石の価値の最大値
    let mut dp_right = vec![vec![0; n+1]; n+1];
    dp_right[0][0] = 0;
    let mut set = BTreeSet::new();
    for i in (1..n+1).rev() {
        set.insert(v[i-1]);
        dp_right[n+1-i][0] = dp_right[n-i][0] + v[i-1];
        for (ind, reduction) in set.iter().enumerate() {
            dp_right[n+1-i][ind + 1] = dp_right[n+1-i][ind] - reduction;
        }
    }


    // 全探索で間に合う
    let mut ans = 0;
    // 進んだ個数
    for le in 0..n+1 {
        for ri in 0..n+1-le {
            // 捨てる個数
            for disl in 0..le+1 {
                for disr in 0..ri+1 {
                    if le + ri + disl + disr > k {continue}
                    ans = max(ans, dp_left[le][disl] + dp_right[ri][disr]);
                }
            }

        }
    }
    println!("{}", ans);
}