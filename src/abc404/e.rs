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
    // 2025-05-03 22:15-22:40 (25min)
    // 2025-05-04 23:04-23:23 (19min)
    // 2025-05-04 02:31-03:01 (30min)
    // 74min
    input! {
        n: usize,
        mut c: [usize; n-1], // 茶碗に書いた数
        mut a: [isize; n-1], // 豆の個数
    }
    c.insert(0, 0);
    a.insert(0, 0);

    // g[L] := {R} L -> R
    let mut g = vec![vec![]; n];

    // rg[R] := {L} L <- R
    let mut rg = vec![vec![]; n];
    for i in 0..n {
        for d in 1..c[i]+1 {
            g[i - d].push(i);
            rg[i].push(i-d);
        }
    }
    let inf = 1 << 60;
    // dp[v] := v にたどり着くのに、必要な手数
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for v in 0..n {
        if a[v] != 0 {
            dp[v] = 0;
        } 
        for &nv in g[v].iter() {
            dp[nv] = min(dp[nv], dp[v] + 1);
        }
    }

    let mut ans = 0;
    for i in (1..n).rev() {
        if a[i] == 0 {continue}

        let mut min_ind = 0;
        let mut min_cost: usize = 1 << 60;
        for &nv in rg[i].iter() {
            if a[nv] != 0 {
                min_ind = nv;
                break
            }
            if dp[nv] < min_cost {
                min_ind = nv;
                min_cost = dp[nv];
            }
        }
        a[min_ind] += a[i];
        a[i] = 0;
        ans += 1;
    }
    println!("{}", ans);
}
