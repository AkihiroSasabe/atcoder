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
    // 2024-07-13 21:45-22:40 (55min)
    // 2024-07-14 9:59-10:32 (33min)
    // 88min
    input! {
        n: usize,
        a: [isize; n]
    }

    let modulus = 998244353;

    // set := 公差
    let mut set = BTreeSet::new();

    // set2 := {key: 数列の値, val: 位置}
    let mut set2 = BTreeMap::new();
    for i in 0..n {
        set2.entry(a[i]).or_insert(vec![]).push(i);
        for j in i+1..n {
            set.insert(a[j] - a[i]);
        }
    }
    let mut diffs = vec![];
    for &diff in set.iter() {
        diffs.push(diff);
    }

    // a[i] から始まる
    // 公差dの数列の個数
    // dpでいけるのでは?
    // 後ろからやるのが、良さそう

    // dp[i][d][k] := a[i]から、公差dで、長さkの数列の総数
    let mut dp = vec![vec![vec![0; n+1]; diffs.len()]; n];

    let mut anss = vec![0; n+1];
    anss[1] = n;
    // anss[2] = n * (n-1) / 2;
    // anss[2] %= modulus;

    // 後ろからやる
    // O(N * (N * (N-1) / 2) * NlogN * K) = O(N^5 logN)
    // 3_276_800_000 = 3 * 10^9 ... 最悪計算量はちょっときついけど、なんやかんやぎりいける?
    let mut v_ps_map: HashMap<isize, Vec<usize>> = HashMap::new();
    for i in (0..n).rev() {
        for j in 0..diffs.len() {
            let d = diffs[j];
            let nv = a[i] + d;
            if let Some(poss) = v_ps_map.get(&nv) {
                for &pos in poss {
                    dp[i][j][2] += 1;
                    dp[i][j][2] %= modulus;
                    for k in 3..n+1 {
                        dp[i][j][k] += dp[pos][j][k-1];
                        dp[i][j][k] %= modulus;
                    }
                }
            }
            
        }
        v_ps_map.entry(a[i]).or_insert(vec![]).push(i);
    }

    for i in 0..n {
        for j in 0..diffs.len() {
            for k in 2..n+1 {
                anss[k] += dp[i][j][k];
                anss[k] %= modulus;
            }
        }
    }
    for i in 1..n+1 {
        print!("{} ", anss[i]);
    }

}