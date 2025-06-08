#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-07 22:33-22:40 (7min)
    // 2025-06-08 09:21-09:34 (13min)
    // Total: 20 min
    input! {
        n: usize,
        x: [isize; n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            ui: Usize1,
            vi: Usize1,
            wi: isize,
        }
        graph[ui].push((vi, wi));
        graph[vi].push((ui, wi));
    }

    let ans = get_subtree_sizes(0, &graph, &x);
    println!("{}", ans);
    


}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
/// 木DP は、葉から根に向かって計算するDP。問題によっては、根の値 dp[root] だけじゃなく、全頂点で、ans = max(ans, dp[v]) をチェックする必要がある。
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<(usize, isize)>>, x: &Vec<isize>) -> isize {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut subtree_sizes = vec![0; n];
    let mut ans = 0;
    dfs(root_v, root_v, &graph, &mut subtree_sizes, x, &mut ans);
    return ans
}
fn dfs(v: usize, parent: usize, graph: &Vec<Vec<(usize, isize)>>, subtree_sizes: &mut Vec<isize>, x: &Vec<isize>, ans: &mut isize) {
    // 問題によって、枝の本数なども数えた方がいい。
    for i in 0..graph[v].len() {
        let (nv, nw) = graph[v][i];
        if nv == parent {continue}
        dfs(nv, v, graph, subtree_sizes, x, ans);
        *ans += subtree_sizes[nv].abs() * nw;
        subtree_sizes[v] += subtree_sizes[nv];

    }
    subtree_sizes[v] += x[v];
}