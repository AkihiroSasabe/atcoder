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
        s: [Chars; n]
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in i+1..n {
            if hantei(&s[i], &s[j], m) {
                // graph[i].insert(j, 0);
                // graph[j].insert(i, 0);
                graph[j].push(i);
                graph[i].push(j);
            }
        }
    }
    let mut seen = vec![false; n];
    let mut ans = false;
    for i in 0..n {
        dfs(i, &graph, &mut seen, 0, n, &mut ans);
        if ans {break}
    }
    if ans {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, depth: usize, n: usize, ans: &mut bool) {
    if depth == n - 1 {
        *ans = true;
    }
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, depth+1, n, ans);
    }
    seen[v] = false;
}

fn hantei(t1: &Vec<char>, t2: &Vec<char>, m: usize) -> bool {
    let mut num = 0;
    for i in 0..m {
        if t1[i] != t2[i] {
            num += 1;
        }
    }

    if num < 2 {
        return true
    }
    else {
        return false
    }
}

