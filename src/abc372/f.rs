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
use rand::Rng;
fn main() {
    // 2024-09-25 18:48-19:46 (58min)
    // evimaさんの解説みた。 0 と m個の辺に繋がれた頂点だけ残す
    // 2024-09-25 19:46-20:41 (55min)
    // Total: 113min
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut set = BTreeSet::new();
    set.insert(0);
    for i in 0..m {
        input!{
            xi: usize,
            yi: usize,
        }
        graph[xi-1].push(yi-1);
        set.insert(xi-1);
        set.insert(yi-1);
    }
    let modulus = 998244353;

    // なぜ、m==0 のときだけ、下のままでうまくいかないのか謎
    if m == 0 {
        println!("1");
        return
    }

    // 頂点0 と m本の辺に繋がれた頂点だけ残す
    let mut vs = vec![];
    let mut v_to_i = vec![0; n];
    for (i, &v) in set.iter().enumerate() {
        vs.push(v);
        v_to_i[v] = i;
    }

    let mut dp = vec![vec![0; vs.len()]; k+1];
    dp[0][0] = 1;
    for ki in 0..k {
        for i in 0..vs.len() {
            let v = vs[i];
            for &nv in graph[v].iter() {
                dp[ki+1][v_to_i[nv]] += dp[ki][v_to_i[v]];
                dp[ki+1][v_to_i[nv]] %= modulus;
            }

            let nv = vs[(i+1) % vs.len()];
            let diff = (nv + n - v) % n;
            if ki + diff > k {continue}
            dp[ki+diff][v_to_i[nv]] += dp[ki][v_to_i[v]];
            dp[ki+diff][v_to_i[nv]] %= modulus;
        }
    }
    
    let mut ans: usize = 0;
    ans += dp[k][0];
    let mut ind = 0;
    for v in 1..n {
        if set.contains(&v) {
            ind += 1;
            ans += dp[k][ind];
            ans %= modulus;
        }
        else {
            let pre_v = vs[ind];
            let diff = v - pre_v;
            if k < diff {continue}
            ans += dp[k-diff][ind];
            ans %= modulus;
        }
    }    
    println!("{}", ans);
}