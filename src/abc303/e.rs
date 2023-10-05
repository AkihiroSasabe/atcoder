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
    // 2023-10-03 Tue. 19:23-22:00 2h37m
    // 2023-10-04 Wed. 21:30-22:00 30min
    // total: 3h7min = 187min

    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for i in 0..n-1 {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i - 1].push(v_i - 1);
        graph[v_i - 1].push(u_i - 1);
        degrees[u_i - 1] += 1;
        degrees[v_i - 1] += 1;
    }
    let mut centers = HashSet::new();
    let mut seen = vec![false; n];
    let mut v_0 = n;
    for v in 0..n {
        if degrees[v] == 1 {
            let next_v = graph[v][0];
            centers.insert(next_v);
            v_0 = v;
        }
    }
    // println!("centers = {:?}, v0 = {}", centers, v_0);

    dfs(v_0, &graph, &mut seen, &mut centers, &degrees, false);
    // println!("centers = {:?}", centers);

    let mut levels = vec![];
    for center in centers.iter() {
        levels.push(degrees[*center]);
    }
    levels.sort();
    for i in levels {
        print!("{} ", i);
    }

}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, centers: &mut HashSet<usize>, degrees: &Vec<usize>, is_bound: bool) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        // vが星同士を繋ぐ頂点(境界)で、かつその隣next_vも境界だった場合、next_vに隣接するvじゃない方の頂点は星の中心
        if is_bound && degrees[next_v] == 2 {
            let cand0 = graph[next_v][0];
            let cand1 = graph[next_v][1];
            // 追加のif条件をマッチガードという。マッチガードがないと、match内で、match外の変数が使えないので注意
            let center = match v {
                x if x == cand0 => cand1,
                y if y == cand1 => cand0,
                _ => 0
            };
            // println!("v ={v}, next_v={}, center={}, cand0={}, cand1={}", next_v, center, cand0, cand1);
            centers.insert(center);
        }

        let is_bound_next = centers.contains(&v) && degrees[next_v] == 2;
        dfs(next_v, graph, seen, centers, degrees, is_bound_next);
    }
}
