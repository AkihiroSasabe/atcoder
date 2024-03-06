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
    // 2024-03-05 21:02-21:13 (11min) ワーシャルフロイド法を貼るだけ。
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            ai: usize,
            bi: usize,
            ci: usize
        }
        graph[ai-1].push(vec![bi-1, ci]);
        // a.push(ai);
        // b.push(bi);
        // c.push(ci);
    }

    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let INF = 1 << 60;
    let mut dp = vec![vec![INF; n]; n];

    // dpの初期化
    for i in 0..n {
        dp[i][i] = 0;
        for j in 0..graph[i].len() {
            dp[i][graph[i][j][0]] = graph[i][j][1];
        }
    }

    let mut ans = 0;
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // k未満の頂点(0-k-1)のみを、中継点として通って良い。
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);

                if dp[i][j] == INF {continue}
                ans += dp[i][j];

                // 例 k = 1の時
                // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
                // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
                // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
                // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
                // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
            }
        }
    }
    println!("{}", ans);



}


// フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
fn floyd_warshall(graph: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
    // 頂点数
    let n = graph.len();
    // dp[i][j]で頂点iから頂点jに行くときの最短距離
    let INF = std::usize::MAX;
    let mut dp = vec![vec![INF; n]; n];

    // dpの初期化
    for i in 0..n {
        dp[i][i] = 0;
        for j in 0..graph[i].len() {
            dp[i][graph[i][j][0]] = graph[i][j][1];
        }
    }
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // k未満の頂点(0-k-1)のみを、中継点として通って良い。
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
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

