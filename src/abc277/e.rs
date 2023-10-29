#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2023-10-29 19:08-19:35 (27min)
    // 頂点数を倍化して、
    // スイッチを押した状態で遷移可能なグラフと、
    // スイッチを押していない状態で遷移可能なグラフを用意するだけ。
    // スイッチが存在する頂点に関しては、その頂点のエッジが、もう一方のグラフにも張れる。
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; 2*n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
            a_i: usize,
        }
        if a_i == 1 {
            graph[u_i - 1].push(v_i - 1);
            graph[v_i - 1].push(u_i - 1);
        }
        else {
            graph[n + u_i - 1].push(n + v_i - 1);
            graph[n + v_i - 1].push(n + u_i - 1);
        }
    }

    input! {
        mut s: [usize; k]
    }
    for i in 0..k {
        s[i] -= 1;
        for j in 0..graph[s[i]].len() {
            let next_v = graph[s[i]][j];
            graph[s[i] + n].push(next_v);
            graph[next_v].push(s[i] + n);
        }
        for j in 0..graph[s[i] + n].len() {
            let next_v = graph[s[i] + n][j];
            graph[s[i]].push(next_v);
            graph[next_v].push(s[i]);
        }
    }
    let distance = bfs(0, &graph);
    // println!("distance = {:?}", distance);
    let mut ans = min(distance[n-1], distance[2*n-1]);
    if ans == 1_000_000_000_000_000_000 {
        println!("-1");
    }
    else {
        println!("{}", ans);
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