#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use core::num;
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
    // 2023-12-08 19:49-20:42 (57min)
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut degree = vec![0; n];
    for i in 0..n-1 {
        input!{
            u_i: usize,
            v_i: usize,
        }
        graph[u_i-1].push(v_i-1);
        graph[v_i-1].push(u_i-1);
        degree[u_i-1] += 1;
        degree[v_i-1] += 1;
    }

    let mut num_leaf = 0;
    let mut seen = vec![false; n];

    let mut lr_list = vec![vec![]; n];
    // println!("ok");
    dfs(0, &graph, &mut seen, &mut lr_list, &degree, &mut num_leaf);
    for i in 0..n {
        println!("{} {}", lr_list[i][0] + 1, lr_list[i][1] + 1)
    }
}


fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, lr_list: &mut Vec<Vec<usize>>, degree: &Vec<usize>, num_leaf: &mut usize) {
    // println!("v = {:?}", v);
    seen[v] = true;
    let mut min_l = graph.len();
    let mut max_r = 0;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, lr_list, degree, num_leaf);
        min_l = min(min_l, lr_list[next_v][0]);
        max_r = max(max_r, lr_list[next_v][1]);
    }

    if v != 0 && degree[v] == 1 {
        lr_list[v] = vec![*num_leaf, *num_leaf];
        *num_leaf += 1;
    }
    else {
        lr_list[v] = vec![min_l, max_r];
    }

}