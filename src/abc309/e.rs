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
    input! {
        n: usize,
        m: usize,
        p: [usize; n-1],
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut hoken = vec![-1; n];
    for i in 0..m {
        input! {
            x_i: usize,
            y_i: isize,
        }
        x.push(x_i-1);
        y.push(y_i);
        hoken[x_i-1] = max(hoken[x_i-1], y_i);
    }

    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        graph[p[i] - 1].push(i+1);
    }

    let mut taisho = vec![false; n];


    let next_length = hoken[0];
    dfs(&graph, 0, &mut taisho, &hoken, next_length);

    let mut ans = 0;
    for i in 0..n {
        if taisho[i] {
            ans += 1;
        }
    }
    // println!("taisho={:?}", taisho);
    println!("{}", ans);

}

fn dfs(graph: &Vec<Vec<usize>>, v: usize, taisho: &mut Vec<bool>, hoken: &Vec<isize>, length: isize) {
    if length >= 0 {
        taisho[v] = true;
    }
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        let next_length = max(length - 1, hoken[next_v]);
        dfs(graph, next_v, taisho, hoken, next_length);
    }

}

