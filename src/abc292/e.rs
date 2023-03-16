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
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut out_edge = vec![0_usize; n];
    let mut in_edge = vec![0_usize; n];
    let mut g_hash = vec![HashMap::new(); n];
    let mut uu = vec![];
    let mut vv = vec![];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        g_hash[u_i-1].insert(v_i-1, 0);
        out_edge[u_i-1] += 1;
        in_edge[v_i-1] += 1;
        uu.push(u_i-1);
        vv.push(v_i-1);
    }

    let mut ans = 0;
    for v in 0..n {
        let mut seen = vec![false; n];
        dfs(&g_hash, v, &mut ans, &mut seen, v);
    }
    println!("{}", ans);

    let tekito = std::u128::MAX;
    println!("{}", tekito);


    // // 計算量が エッジ数(外側ループwhile) x 頂点数(内側ループ)となる。
    // // エッジは増えていくので、最大でN*(N-1)/2個になってしまう。これではO(N^3=8*10^9)でTLE
    // // v0->v1->v2のとき、v0->v2が存在するようにする
    // // ei := edge の index
    // let mut ei = 0;
    // while ei < uu.len()  {
    //     let v0 = uu[ei];
    //     let v1 = vv[ei];
    //     for j in 0..graph[v1].len() {
    //         let v2 = graph[v1][j];
    //         if !g_hash[v0].contains_key(&v2) && v2 != v0 {
    //             uu.push(v0);
    //             vv.push(v2);
    //             ans += 1;
    //             g_hash[v0].insert(v2, 0);
    //             graph[v0].push(v2);
    //             // println!("add: {} -> {}", v0+1, v2+1);
    //         }
    //     }
    //     ei += 1;
    // }
}

// root_vから到達可能でかつ距離が2以上の頂点の個数を数える。
fn dfs(graph: &Vec<HashMap<usize, usize>>, v: usize, ans: &mut usize, seen: &mut Vec<bool>, root_v: usize) {
    seen[v] = true;
    for (&next_v, _) in &graph[v] {
        if seen[next_v] {continue}
        if !graph[root_v].contains_key(&next_v) {
            *ans += 1;
        }
        dfs(graph, next_v, ans, seen, root_v);
    }
}