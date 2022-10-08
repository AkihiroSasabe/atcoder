use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        xx: usize,
        yy: usize,
    }
    let x = xx - 1;
    let y = yy - 1;
    let mut graph = vec![vec![]; n];

    for i in 0..(n-1) {
        input! {
            u_i: usize,
            v_i: usize,
        }
        graph[u_i - 1].push(v_i - 1);
        graph[v_i - 1].push(u_i - 1);
    }

    let mut seen = vec![false; n];
    let mut path = vec![];
    dfs(&graph, x, &mut seen, y, &mut path);
    
}

fn dfs(g: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, y: usize, path: &mut Vec<usize>) {
    path.push(v);
    if path[path.len() - 1] == y {
        for i in 0..path.len() {
            print!("{} ", path[i] + 1);
        }
        return;
    }
    seen[v] = true;
    for next_v in g[v].iter() {
        if seen[*next_v] {continue}
        dfs(g, *next_v, seen, y, path);
    }
    path.pop();
}