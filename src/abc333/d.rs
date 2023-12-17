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
    input! {
        n: usize,
    }
    let mut degrees = vec![0; n];
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
        degrees[ui-1] += 1;
        degrees[vi-1] += 1;
    }

    // 各頂点の部分木の大きさを調べる
    let subtree_sizes = get_subtree_sizes(0, &graph, &degrees);
    // println!("subtree_sizes = {:?}", subtree_sizes);

    // 頂点0に隣接する頂点の部分木のサイズを格納したlist
    let mut sizes_around_0 = vec![];
    for i in 0..graph[0].len() {
        let nv = graph[0][i];
        sizes_around_0.push(subtree_sizes[nv]);
    }

    let mut ans = 0;
    if sizes_around_0.len() != 0 {
        // 部分木のサイズが一番大きい奴だけ除いた、部分木のサイズの和を取る
        sizes_around_0.sort();
        for i in 0..sizes_around_0.len()-1 {
            ans += sizes_around_0[i];
        }
    }
    // 根 (頂点1)を切断する分を加算
    ans += 1;

    println!("{}", ans);

}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
/// degrees[v] := 頂点vの次数
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>, degrees: &Vec<usize>) -> Vec<usize> {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut subtree_sizes = vec![0; n];
    let mut seen = vec![false; n];
    dfs(root_v, &graph, &mut seen, &mut subtree_sizes, &degrees);
    return subtree_sizes
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, subtree_sizes: &mut Vec<usize>, degrees: &Vec<usize>) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, subtree_sizes, degrees);
        subtree_sizes[v] += subtree_sizes[next_v];
    }
    subtree_sizes[v] += 1;
}