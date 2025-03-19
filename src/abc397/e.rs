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
    input! {
        n: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n*k];
    for i in 0..n*k-1 {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
    }

    if get_subtree_sizes(0, &graph, k) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
/// 木DP は、葉から根に向かって計算するDP。問題によっては、根の値 dp[root] だけじゃなく、全頂点で、ans = max(ans, dp[v]) をチェックする必要がある。
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>, k: usize) -> bool {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut subtree_sizes = vec![0; n];
    let mut seen = vec![false; n];
    let is_ok = dfs(root_v, &graph, &mut seen, &mut subtree_sizes, k);

    if is_ok && subtree_sizes[0] == 0 {
        return true
    }
    else {
        return false
    }
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, subtree_sizes: &mut Vec<usize>, k: usize) -> bool {
    seen[v] = true;
    let mut num_children = 0;
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if seen[nv] {continue}
        let is_ok = dfs(nv, graph, seen, subtree_sizes, k);
        if !is_ok {
            return false
        }
        if subtree_sizes[nv] > 0 {
            num_children += 1;
        }
        subtree_sizes[v] += subtree_sizes[nv];
    }
    subtree_sizes[v] += 1;
    
    if subtree_sizes[v] > k {
        return false
    }
    else if subtree_sizes[v] == k {
        if num_children < 3 {
            subtree_sizes[v] = 0;
            return true
        }
        else {
            return false
        }
    }
    else {
        return num_children < 2
    }
}