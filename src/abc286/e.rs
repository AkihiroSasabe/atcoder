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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize
    }
    let mut u = vec![];
    let mut v = vec![];
    for i in 0..q {
        input! {
            u_i: usize,
            v_i: usize,
        }
        u.push(u_i-1);
        v.push(v_i-1);
    }

    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                graph[i].push(j);
            }
        }
    }

}

fn dfs(v: usize, graph: &mut Vec<Vec<usize>>, dist: usize, seen: &mut Vec<bool>) {

}

// // フロイド・ワーシャル法で、全頂点対間の距離をO(V^3)で最小化 (全点対間最短経路問題)
// fn floyd_warshall(graph: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
//     // 頂点数
//     let n = graph.len();
//     // dp[i][j]で頂点iから頂点jに行くときの最短距離
//     let INF = std::usize::MAX;
//     let mut dp = vec![vec![vec![INF, 0]; n]; n];

//     // dpの初期化
//     for i in 0..n {
//         dp[i][i] = 0;
//         for j in 0..graph[i].len() {
//             dp[i][graph[i][j][0]] = graph[i][j][1];
//         }
//     }
    
//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 // k未満の頂点(0-k-1)のみを、中継点として通って良い。
//                 if dp[i][j][0] > dp[i][k][0] + dp[k][j][0] {
//                     dp[i][j][0] = dp[i][k][0] + dp[k][j][0];
//                 }
//                 dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
//                 // 例 k = 1の時
//                 // dp[0][0] = min(dp[0][0], dp[0][1] + dp[1][0]);
//                 // dp[0][1] = min(dp[0][1], dp[0][1] + dp[1][1]);
//                 // dp[0][2] = min(dp[0][2], dp[0][1] + dp[1][2]);
//                 // dp[0][3] = min(dp[0][3], dp[0][1] + dp[1][3]);
//                 // dp[0][4] = min(dp[0][4], dp[0][1] + dp[1][4]);
//             }
//         }
//     }
//     return dp
// }
