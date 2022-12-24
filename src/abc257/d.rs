use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut p = vec![];

    for _ in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
            p_i: isize
        }
        x.push(x_i);
        y.push(y_i);
        p.push(p_i);
    }

    // フロイド・ワーシャル法で解く
    // 全点対間最短経路問題
    let mut graph = vec![vec![]; n];

    // let mut s = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            let dist = ((x[i] - x[j]).abs() + (y[i] - y[j]).abs());
            let mut kuriage = 0;
            if dist % p[i] != 0 {
                kuriage = 1;
            }
            let weight = dist / p[i] + kuriage;
            // println!("i:{} => j: {}, dist:{}, p[i]: {}, weight: {}", i, j, dist, p[i], weight);
            graph[i].push(vec![j, weight as usize]);
        }
    }
    let dp = pseudo_floyd_warshall(&graph);
    let mut ans = std::usize::MAX;
    for i in 0..n {
        let mut candiate = 0;
        for j in 0..n {
            candiate = max(candiate, dp[i][j]);
        }
        ans = min(ans, candiate);
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


// ワーシャルフロイドをこの問題の為にアレンジ
fn pseudo_floyd_warshall(graph: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
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
                // ワーシャル・フロイドは以下
                // dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);

                // 本問題のケース
                dp[i][j] = min(dp[i][j], max(dp[i][k], dp[k][j]));
            }
        }
    }
    return dp
}