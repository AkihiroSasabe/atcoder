#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::iter::Cycle;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;


fn main() {
    // 2023-12-08 12:48-13:11 (23min)
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut reverse_graph = vec![vec![]; n];

    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i - 1].push(v_i - 1);
        reverse_graph[v_i - 1].push(u_i - 1);
    }

    // ループを超頂点とするとか?
    // 強連結成分分解
    let scc_list = decompositon_of_strongly_connected_components(&graph, &reverse_graph, n);

    let mut cycles = vec![];
    let mut to_loop = vec![false; n];
    for i in 0..scc_list.len() {
        for j in 1..scc_list[i].len() {
            let v = scc_list[i][j];
            cycles.push(v);
            to_loop[v] = true;
        }
    }

    let mut seen = vec![false; n];
    for i in 0..cycles.len() {
        let v = cycles[i];
        if seen[v] {continue}
        dfs(&reverse_graph, v, &mut seen, &mut to_loop)
    }
    let mut ans = 0;
    for i in 0..n {
        if to_loop[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
    // println!("{:?}", scc_list);

}

fn dfs(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, to_loop: &mut Vec<bool>) {
    seen[v] = true;
    to_loop[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs(graph, *next_v, seen, to_loop);
    }
}


// 1回目のDFS
fn dfs1(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, sorted_list: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs1(graph, *next_v, seen, sorted_list);
    }
    sorted_list.push(v);
}

// 2回目のDFS。トポロジカルソートした番号の逆順から攻める。
fn dfs2(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, scc: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs2(graph, *next_v, seen, scc);
    }
    scc.push(v);
}


// 強連結成分分解 (蟻本p285~p288) 計算量O(E)
fn decompositon_of_strongly_connected_components(graph: &Vec<Vec<usize>>, reverse_graph: &Vec<Vec<usize>>, v_num: usize) -> Vec<Vec<usize>>{

    // 1回目のDFS: トポロジカルソートする
    let mut reverse_topological_sorted_list = vec![];
    let mut seen = vec![false; v_num];
    for v in 0..v_num {
        if seen[v] {continue}
        dfs1(graph, v, &mut seen, &mut reverse_topological_sorted_list);
    }

    // 2回目のDFS: グラフの辺を逆向きにして、たどり着ける頂点を強連結成分としてまとめる
    let mut scc_list = vec![];
    let mut seen = vec![false; v_num];
    reverse_topological_sorted_list.reverse();
    let topological_sorted_list = reverse_topological_sorted_list;
    for v in topological_sorted_list {
        if seen[v] {continue}
        let mut strongly_connected_components = vec![];
        dfs2(&reverse_graph, v, &mut seen, &mut strongly_connected_components);
        scc_list.push(strongly_connected_components);
    }

    return scc_list
}
