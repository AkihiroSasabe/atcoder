#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
fn main() {
    // 2024-11-16 -13:20-15:30 (10min)
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
    }

    // DFS か?
    let mut ans = 0;
    let mut seen = vec![false; n];
    dfs(0, &graph, &mut seen, &mut ans, 0);
    println!("{}", ans);

}


fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, ans: &mut usize, depth: usize) {
    if depth == graph.len() - 1 {
        *ans += 1;
        return
    }
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, ans, depth + 1);
    }
    seen[v] = false;
}