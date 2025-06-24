#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, vec};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{traits::ops::checked, BigUint, Integer, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-14 21:48-22:40 (52min)
    // 2025-06-14 23:50-24:01 (11min)
    // 63min (F問題を考察していた時間も含む)
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
            wi: usize,
        }
        graph[ui].push((vi, wi));
        edges.push((ui, vi, wi));
    }

    // ベルマンフォードもどきで解いた。普通にベルマンフォードすると、TLEするので枝狩りも必要（コンテスト中は枝狩りの実装まで間に合わなかった）。
    // solve_real_time(n, m, graph, edges);

    // 公式解説の頂点倍加
    solve_after_contest(n, m, graph, edges);
}

fn solve_after_contest(n: usize, m: usize, graph: Vec<Vec<(usize, usize)>>, edges: Vec<(usize, usize, usize)>) {
    // 公式解説の頂点倍加
    // 2025-06-15 11:09-11:22 (13min)

    // g2 := 2^10 = 1024 倍に頂点を増やしたグラフ
    let mut g2 = vec![vec![]; n*(1<<10)];
    for &(v, nv, w) in edges.iter() {
        for mask in (0..1<<10) {
            let n_mask = mask ^ w;
            let v2 = v * (1<<10) + mask;
            let nv2 = nv * (1<<10) + n_mask;
            g2[v2].push(nv2);
        }
    }

    fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
        // v0が始点
        let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
        let mut queue = VecDeque::new();
        let mut distance = vec![init_distance; graph.len()];
        distance[v0] = 0;
        queue.push_back(v0);
        while queue.len() > 0 {
            let v = queue.pop_front().unwrap();
            for i in 0..graph[v].len() {
                let nv = graph[v][i];
                if distance[nv] != init_distance {continue}
                distance[nv] = distance[v] + 1;
                queue.push_back(nv);
            }
        }
        return distance
    }
    let dists = bfs(0, &g2);
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18

    let mut ans = init_distance;
    for mask in 0..1<<10 {
        let v = (n-1)*(1<<10) + mask;
        if dists[v] != init_distance {
            ans = min(ans, mask);
        }
    }

    if ans == init_distance {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }



}





fn solve_real_time(n: usize, m: usize, graph: Vec<Vec<(usize, usize)>>, edges: Vec<(usize, usize, usize)>) {
    // ベルマンフォードもどきで解いた。普通にベルマンフォードすると、TLEするので枝狩りも必要（コンテスト中は枝狩りの実装まで間に合わなかった）。


    // dp[v] := v が取りうる xor の集合 (未使用)
    let mut dp = vec![vec![]; n];
    dp[0].push(0); // スタート地点は0

    // dp_seen[v] := v が取りうる xor の集合 (使用済み)
    let mut dp_seen = vec![vec![]; n];

    let mut seen = vec![vec![false; 1<<10]; n];
    seen[0][0] = true;

    for num_update in 0..n+2 {
        let mut diff_dp = vec![vec![]; n];
        for v in 0..n {
            for j in 0..graph[v].len() {
                let (nv, nw) = graph[v][j];
                for &xor in dp[v].iter() {
                    let n_xor = xor ^ nw;
                    if seen[nv][n_xor] {continue;}
                    seen[nv][n_xor] = true;
                    diff_dp[nv].push(n_xor);
                }
            }
        }
        for v in 0..n {
            for &xor in dp[v].iter() {
                dp_seen[v].push(xor);
            }
            dp[v] = vec![];
            swap(&mut dp[v], &mut diff_dp[v])
        }
    }
    let inf = 1 << 60;
    let mut ans = inf;
    for &xor in &dp_seen[n-1] {
        ans = min(ans, xor);
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }

        




    
    // ベルマン
    // let INF = 1 << 60;
    // let mut dists = vec![-INF; n];
    // dists[0] = 0;
    // for num_update in 0..n {
    //     for i in 0..m {
    //         let (ui, vi, wi) = edges[i];
    //         if dists[vi] < dists[ui] + wi {
    //             if num_update == n - 1 && vi == n - 1 {
    //                 println!("inf");
    //                 return
    //             }
    //             dists[vi] = dists[ui] + wi;
    //         }
    //         // dists[vi] = max(dists[vi], dists[ui] + wi);
    //     }
    // }

}