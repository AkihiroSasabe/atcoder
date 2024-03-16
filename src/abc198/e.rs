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
    // 2024-03-16 13:06-13:30 (24min)
    // 2024-03-16 14:42-14:55 (13min)
    // total: 37min
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{ 
            ai: usize,
            bi: usize,
        }
        graph[ai-1].push(bi-1);
        graph[bi-1].push(ai-1);
    }
    let mut seen: Vec<bool> = vec![false; n];
    let mut seen_x: Vec<bool> = vec![false; n];
    let mut seen_color = vec![false; 100_001];
    dfs(0, &graph, &mut seen, &mut seen_color, &mut seen_x, &c);
    for i in 0..n {
        if seen_x[i] {
            println!("{}", i+1);
        }
    }

}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, seen_color: &mut Vec<bool>, seen_x: &mut Vec<bool>, c: &Vec<usize>) {
    // println!("v = {}", v + 1);
    seen[v] = true;
    let is_first_color = !seen_color[c[v]];
    if !seen_color[c[v]] {
        seen_x[v] = true;
    }
    seen_color[c[v]] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, seen_color, seen_x, c);
    }
    // 帰り際に戻しておく
    if is_first_color {
        seen_color[c[v]] = false;
    }
}