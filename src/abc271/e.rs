use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, BTreeMap, HashSet};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-11-01 20:34-21:29 (55min)
    // 2023-11-01 21:29-22:00 (降参, 解説を見る。競プロフレンズいわく、ベルマンフォード法に似ているらしい。)
    // 2023-11-02 20:15-20:31 (16min)
    // 1h11m
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
            c_i: usize,
        }
        a.push(a_i - 1);
        b.push(b_i - 1);
        c.push(c_i);
    }
    let mut e = vec![];
    for i in 0..k {
        input!{ 
            e_i: usize,
        }
        e.push(e_i - 1);
    }

    let inf: usize = 1_000_000_000_000_000_000;
    // dp[v] := 原点0からvまでの最短距離
    // dp[i][v] := E0..Eiまでの部分列で、原点0からvまでの最短距離 <- inplaceに計算できるので、配列はdp[v]だけでいい。
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..k {
        dp[b[e[i]]] = min(dp[b[e[i]]], dp[a[e[i]]] + c[e[i]]);
    }
    let mut ans = inf;
    ans = min(ans, dp[n-1]);


    // let mut starts = vec![vec![]; n];
    // for i in 0..k {
    //     let edge = e[i];
    //     starts[a[edge]].push(i);
    // }

    // let inf: usize = 1_000_000_000_000_000_000;
    // // dp[i] := 経路iを通ったときに必要な距離
    // let mut dp = vec![inf; k];
    // for i in 0..k {
    //     let edge = e[i];
    //     if a[edge] == 0 {
    //         dp[i] = min(dp[i], c[edge]);
    //     }
    //     if dp[i] != inf {
    //         // iより大きくて、b[edge]から始まるエッジを追加していく
    //         let index = starts[b[edge]].upper_bound(&i);
    //         for vvv in index..starts[b[edge]].len() {
    //             let next_ind = starts[b[edge]][vvv];
    //             let next_edge = e[next_ind];
    //             dp[next_ind] = min(dp[next_ind], dp[i] + c[next_edge]);
    //         }
    //     }
    // }

    // let mut ans = inf;
    // for i in 0..k {
    //     let edge = e[i];
    //     if b[edge] == n - 1 {
    //         ans = min(ans, dp[i]);
    //     }
    // }
    if ans == inf {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }

}
