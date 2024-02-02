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
    // 2024-01-26 21:21-21:59 (38min)
    // 2024-01-27 18:02-18:11 (9min)
    // total: 47min
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for i in 0..n-1 {
        input!{
            uii: usize,
            vii: usize
        }
        let ui = uii - 1;
        let vi = vii - 1;
        graph[ui].push(vi);
        graph[vi].push(ui);
        degrees[ui] += 1;
        degrees[vi] += 1;
    }
    // 頂点0から、各頂点までの距離
    let distances = bfs(0, &graph);

    // sum_list[i] := ∑ j=[0,N-1]​ dis(i,j) 
    let mut sum_list = vec![0; n];
    
    // 頂点0の Σdis だけ先に求める 
    let mut sum = 0;
    for i in 0..n {
        sum += distances[i];
    }
    sum_list[0] = sum;

    // 根を頂点0とする、各頂点の部分木の大きさ
    let subtree_sizes = get_subtree_sizes(0, &graph, &degrees);

    // 隣り合う頂点の Σdis の大きさは、隣り合う頂点の部分木の大きさの差で決まる。 (詳細は、dfs2の実装を参考)
    dfs2(0, &graph, &mut sum_list, &subtree_sizes);
    for i in 0..n {
        println!("{}", sum_list[i]);
    }



}


/// sum_list[i] (i=1,2,...n-1) を求めていくためのdfs
fn dfs2(v: usize, graph: &Vec<Vec<usize>>, sum_list: &mut Vec<usize>, subtree_sizes: &Vec<usize>) {
    let n = graph.len();

    // 木なので、根0からdfsすると、基本的に親vから子nvへしか遷移しない。
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if sum_list[nv] != 0 {continue}

        // 子の頂点の Σdis の大きさは、子の部分木の大きさ　と　子以外の頂点数　の差　と　親のΣdis で決まる。
        // 理由は以下の2つの事実から分かる。
        // (1)子頂点の部分木の各頂点の距離は、nvの方がvより1個分近い。
        // (2)一方、それ以外の各頂点の距離は、nvの方がvより1個分遠い。
        sum_list[nv] = sum_list[v] + (n - subtree_sizes[nv]) - subtree_sizes[nv]; 
        dfs2(nv, graph, sum_list, subtree_sizes);
    }  
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
    // 部分木の大きさを求めるためのdfs
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, subtree_sizes, degrees);
        subtree_sizes[v] += subtree_sizes[next_v];
    }
    subtree_sizes[v] += 1;
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