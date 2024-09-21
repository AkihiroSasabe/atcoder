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
    // 2024-09-19 20:05-20:42 (37min)
    input! {
        n: usize,
        m: usize,
        r: usize,
        mut rs: [usize; r],
    }
    rs.iter_mut().for_each(|x| *x-=1);
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ai: usize,
            bi: usize,
            ci: usize,
        }
        graph[ai-1].push((bi-1, ci));
        graph[bi-1].push((ai-1, ci));
    }
    // println!("rs = {:?}", rs);

    // ワーシャルフロイドのニオい
    let mut ans: usize = std::usize::MAX;
    let dp = floyd_warshall(&graph);
    // for i in 0..n {
    //     println!("dp[{i}] = {:?}", dp[i]);
    // }

    for perm in (0..r).permutations(r) {
        let mut cand = 0;
        for i in 1..r {
            let pv = rs[perm[i-1]];
            let v = rs[perm[i]];
            // println!("pv = {}, v = {:?}", pv, v);
            let dist = dp[pv][v];
            cand += dist;
        }
        ans = min(ans, cand);
    }
    println!("{}", ans);
}

// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
fn floyd_warshall<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<T>> 
    where T: 
        Copy + 
        Ord +
        std::cmp::PartialEq + 
        std::ops::Div<Output = T> +
        num::Zero +
        num::One +
        num::Bounded // max_value() で要る
{
    // 頂点数
    let n = graph.len();

    // 初期化のために、任意の型に対応した、 0 と max / 2 が必要。
    let zero: T     = T::zero();
    let one: T      = T::one();
    let two: T      = one + one;
    let inf: T      = T::max_value() / two;

    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let mut dp: Vec<Vec<T>> = vec![vec![inf; n]; n];

    // dpの初期化
    for v in 0..n {
        // 同一頂点への移動は0
        dp[v][v] = zero;
        for i in 0..graph[v].len() {
            // 直接遷移可能な頂点への移動を格納
            let nv = graph[v][i].0;
            let dist = graph[v][i].1;
            dp[v][nv] = dp[v][nv].min(dist);
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // dp[i][j] := i -> j へ、k未満の頂点(0 ~ k-1)のみを、中継点として通って良い。
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                // 例 k = 1の時
                // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
                // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }
    return dp
}

