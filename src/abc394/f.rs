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
    // 2025-02-22 21:31-24:52 (3h21min)
    input! {
        n: usize
    }

    // 公式解説
    // dp[v] := （v を含む） v の子孫のみを用いる部分木であって、 v の親を追加することで広義アルカンになるようなものの頂点数の最大値
    // 競プロフレンズ
    // dp[v] := dp[v]=vの部分木で、vの親を入れるとアルカンになるような頂点数最大の部分グラフ

    let mut degs = vec![0; n];
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            xi: Usize1,
            yi: Usize1,
        }
        graph[xi].push(yi);
        graph[yi].push(xi);
        degs[xi] += 1;
        degs[yi] += 1;
    }

    // println!("degs = {:?}", degs);

    // 最大次数を計算
    let mut max_deg = 0;
    let mut root = 0;
    for i in 0..n {
        if max_deg <= degs[i] {
            root = i;
            max_deg = degs[i];
        }
    }
    if max_deg < 4 {
        println!("-1");
        return
    }

    let ans = get_subtree_sizes(root, &graph, &degs);
    // println!("subtree_sizes = {:?}", subtree_sizes);
    println!("{}", ans);

    // 
}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>,  degs: &Vec<usize>) -> usize {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut subtree_sizes = vec![0; n];
    let mut seen = vec![false; n];
    let mut ans = 0;
    dfs(root_v, root_v, &graph, &mut seen, &mut subtree_sizes, degs, &mut ans);
    return ans
}


fn dfs(root_v: usize, v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, subtree_sizes: &mut Vec<usize>, degs: &Vec<usize>, ans: &mut usize) {
    seen[v] = true;
    if degs[v] >= 4 {
        let mut list = vec![];
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if seen[nv] {continue}
            dfs(root_v, nv, graph, seen, subtree_sizes, degs, ans);            
            list.push(subtree_sizes[nv]);
            // subtree_sizes[v] += subtree_sizes[nv];
        }
        list.sort();
        list.reverse();
        let mut num = 3;
        for i in 0..num {
            subtree_sizes[v] += list[i];
        }
        subtree_sizes[v] += 1;
        if list.len() > 3 {
            *ans = max(*ans, subtree_sizes[v] + list[3]);
        }
    }
    else {
        subtree_sizes[v] = 1;

    }
}