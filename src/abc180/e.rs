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
    // 2024-03-23 15:24-16:03 // 29min 降参
    // 2024-03-23 16:03-17:11 // 18 min 
    // Total 47min
    input! {
        n: usize
    }
    let mut x: Vec<isize> = vec![];
    let mut y: Vec<isize> = vec![];
    let mut z: Vec<isize> = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
            zi: isize,
        }
        x.push(xi);
        y.push(yi);
        z.push(zi);
    }

    // dist[i][j] := iとjの距離
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            dist[i][j] = get_cost(i, j, &x, &y, &z);
        }
    }

    // 巡回セールスマン問題
    // dp[S][v] := 現在頂点vにいて、訪問済みの頂点集合がS
    // O(2^N * N^2) = 37_879_808 = 3 * 10^7 < 10^8
    let INF = 1 << 62;
    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[1 << 0][0] = 0; // どこから初期化してもいい。
    for bit in 0..1<<n {
        for v in 0..n {
            for nv in 0..n {
                if bit & (1 << v) != 0 {
                    // println!("v, nv, bit = {:?}, {}, ({:0b}, {})", v, nv, bit, bit);
                    dp[bit | (1 << nv)][nv] = min(dp[bit | (1 << nv)][nv], dp[bit][v] + dist[v][nv]);
                }
            }
        }
    }
    let mut ans = INF;
    for v in 1..n {
        ans = min(ans, dp[(1 << n) - 1][v] + dist[v][0]);
    }
    println!("{}", ans);

    // 7_117_462
    // 6_519_344

    // 2^17 = 131_072 = 10^5


    // 2<=N<= 17
    // 16! = 1_307_674_368_000
    // zでコストが決まる。低いところから、高いところに登ると、よけいにコストがかかる
    // UFT

}

fn get_cost(s: usize, t: usize, x: &Vec<isize>, y: &Vec<isize>, z: &Vec<isize>) -> isize {
    let xs = x[s];
    let ys = y[s];
    let zs = z[s];

    let xt = x[t];
    let yt = y[t];
    let zt = z[t];

    let xc = (xt - xs).abs();
    let yc = (yt - ys).abs();
    let zc = max(0, zt - zs);

    return xc + yc + zc

}