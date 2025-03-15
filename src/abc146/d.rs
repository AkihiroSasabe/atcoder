#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-03-07 12:37-13:09 (32min)
    input! {
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut graph  = vec![vec![]; n];
    let mut edges = BTreeMap::new();
    for i in 0..n-1 {
        input!{
            ai: Usize1,
            bi: Usize1,
        }
        a.push(ai);
        b.push(bi);
        graph[ai].push(bi);
        graph[bi].push(ai);
        edges.insert((min(ai,bi), max(ai,bi) ),i);
    }

    // vertice[v] := {iro: nv}
    let mut edge_color = vec![0; n-1];
    let mut num = 0;
    let vertice = get_subtree_sizes(0, &graph);

    for v in 0..n {
        for (&iro, &nv) in vertice[v].iter() {
            if let Some(&edge_index) = edges.get(&(min(v, nv), max(v, nv))) {
                edge_color[edge_index] = iro;
            }
            num = max(num, iro);
        }
    }

    println!("{}", num);
    for i in 0..n-1 {
        println!("{}", edge_color[i]);
    }
}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
/// 木DP は、葉から根に向かって計算するDP。問題によっては、根の値 dp[root] だけじゃなく、全頂点で、ans = max(ans, dp[v]) をチェックする必要がある。
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>) -> Vec<BTreeMap<usize,usize>> {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    // let mut subtree_sizes = vec![0; n];
    let mut seen = vec![false; n];
    let mut vertice = vec![BTreeMap::new(); n];
    dfs(root_v, &graph, &mut seen, &mut vertice);
    return vertice
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, vertice: &mut Vec<BTreeMap<usize,usize>>) {
    seen[v] = true;

    let mut next_color = 1;
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if seen[nv] {continue}
        while vertice[v].contains_key(&next_color) {
            next_color += 1;
        }
        vertice[v].insert(next_color, nv);
        vertice[nv].insert(next_color, v);
        dfs(nv, graph, seen, vertice);
    }
}