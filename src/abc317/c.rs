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
        m: usize,
        
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
            c_i: usize,
        }
        graph[a_i - 1].push(vec![b_i-1, c_i]);
        graph[b_i - 1].push(vec![a_i-1, c_i]);
    }

    let mut ans = 0;
    let mut seen = vec![false; n];
    for start in 0..n {
        dfs(start, &graph, &mut seen, 0, &mut ans);
    }
    println!("{}", ans);
}


fn dfs(v: usize, graph: &Vec<Vec<Vec<usize>>>, seen: &mut Vec<bool>, temp: usize, max_dist: &mut usize) {
    // println!("temp={}, max_dist={}", temp, max_dist);
    seen[v] = true;
    *max_dist = max(*max_dist, temp);
    for i in 0..graph[v].len() {
        let next_v = graph[v][i][0];
        let next_len = graph[v][i][1];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, temp + next_len, max_dist);
    }
    seen[v] = false;
}