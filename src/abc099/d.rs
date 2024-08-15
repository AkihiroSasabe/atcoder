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
    // 2024-08-14 11:33-11:56 (33min)
    input! {
        n: usize,
        c: usize,
        d: [[usize; c]; c],
        mut a: [[usize; n]; n],
    }

    // C_P_3 = 30 * 29 * 28 = 24360
    // N^2 = 500 * 500 = 250_000
    // C_P_3 * N^2 = 24360 * 250_000 = 6_090_000_000 > 6 * 10^9 // これはTLE

    //O(N^2 + 3*C * C_P_3) で解ける。

    let mut nums = vec![vec![0; c]; 3];
    for i in 0..n {
        for j in 0..n {
            a[i][j] -= 1;
            nums[(i + j) % 3][a[i][j]] += 1;
        }
    }

    let mut ans: usize = 1000 * 500 * 500;
    for perm in (0..c).permutations(3) {

        let mut cand = 0;
        for source in 0..3 {
            for source_color in 0..c {
                let cost = d[source_color][perm[source]]* nums[source][source_color];
                cand += cost;
            }
        }
        ans = min(ans, cand);
    }
    println!("{}", ans);

    

}