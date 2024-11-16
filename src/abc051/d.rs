#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-15 18:01-18:29 (28min)
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut edges: Vec<(usize, usize, usize)> = vec![];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
            ci: usize,
        }
        graph[ui-1].push((vi-1, ci));
        graph[vi-1].push((ui-1, ci));
        edges.push((ci, ui-1, vi-1));
    }

    // my_solve(n, m, graph, edges);
    // educational_slow_solve(n, m, graph, edges);
    educational_first_solve(n, m, graph, edges);

}

fn educational_slow_solve(n: usize, m: usize, graph: Vec<Vec<(usize, usize)>>, mut edges: Vec<(usize, usize, usize)>) {

    let dp = floyd_warshall(&graph);

    let mut ans = 0;
    for i in 0..m {
        let (ci, ui, vi) = edges[i];
        let mut is_need = false;
        for s in 0..n {
            if is_need {break}
            for t in s+1..n {
                // 頂点 ui と頂点 vi を結ぶ辺が頂点 s から頂点 t への最短経路に含まれるとき
                if dp[s][t] == dp[s][ui] + ci + dp[vi][t] || dp[s][t] == dp[s][vi] + ci + dp[ui][t] {
                    is_need = true;
                    break
                }
            }
        }
        if !is_need {
            ans += 1;
        }
    }
    println!("{}", ans);
}


fn educational_first_solve(n: usize, m: usize, graph: Vec<Vec<(usize, usize)>>, mut edges: Vec<(usize, usize, usize)>) {

    let dp = floyd_warshall(&graph);

    let mut ans = m;
    for i in 0..m {
        let mut is_shortest = false;
        let (ci, ui, vi) = edges[i];

        for s in 0..n {
            if dp[s][ui] + ci == dp[s][vi] {
                is_shortest = true;
            }
        }
        if is_shortest {ans -= 1}
    }
    println!("{}", ans);
}

fn my_solve(n: usize, m: usize, graph: Vec<Vec<(usize, usize)>>, mut edges: Vec<(usize, usize, usize)>) {
    // ABC51-Dで、「どの異なる 2 頂点間の、どの最短経路にも含まれない辺の数を求めてください。」とあるのですが、頂点uと頂点vの最短経路が複数あって場合は、そのうちの1本の最短経路が辺eを含んでいる場合でも、eは「どの異なる 2 頂点間の、どの最短経路にも含まれない辺」とすべきでしょうか?
    // 例えば、以下のような入力例の場合、頂点1と頂点3の最短経路長は2で経路は2本（1->2->3と1->3）ありますが、1->3 という辺は「どの異なる 2 頂点間の、どの最短経路にも含まれない辺」という扱いになるのでしょうか?
    // 3 3
    // 1 2 1
    // 1 3 2
    // 2 3 1
    let dp = floyd_warshall(&graph);
    let mut ans = 0;
    for i in 0..m {
        let (ci, ui, vi) = edges[i];
        if dp[ui][vi] < ci {
            ans += 1;
        }
    }
    println!("{}", ans);
}


// 類題: abc73_D: https://atcoder.jp/contests/abc073/tasks/abc073_d
// 類題: abc257_D: https://github.com/AkihiroSasabe/atcoder/blob/main/src/abc257/d.rs
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
    let ten: T      = two + two + two + two + two;
    let inf: T      = T::max_value() / ten;
    // let INF: usize = usize::MAX / 10;


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


