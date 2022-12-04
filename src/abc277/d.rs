#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut tinko = HashMap::new();
    for i in 0..n {
        if tinko.contains_key(&a[i]) {
            *tinko.get_mut(&a[i]).unwrap() += 1_usize;
        }
        else {
            tinko.insert(a[i], 1);
        }
    }
    let mut hash_map = HashMap::new();

    for (k, v) in &tinko {
        if *k == (*k+1)  % m && tinko.contains_key(&((*k+1)  % m)) {
            hash_map.entry(*k).or_insert(vec![]).push((*k+1) % m);
            hash_map.entry((*k+1) % m).or_insert(vec![]).push(*k);
        }
    }
    for i in 0..n {
        
    }
    dfs(&hash_map, &mut ans, 0, 0, &mut seen);

    let mut ans = 0;


}


fn dfs(graph: &HashMap<usize, Vec<usize>>, ans: &mut usize, v: usize, parent: usize, seen: &mut HashMap<usize, bool>) {
    seen.insert(v, true);
    if !graph.contains_key(&v) {
        return
    }
    for i in 0..graph[&v].len() {
        let v_next = graph[&v][i];
        if v_next == parent {continue}
        if seen.contains_key(&v_next) {continue}
        if *ans < v_next {
            *ans = v_next;
        }
        dfs(graph, ans, v_next, v, seen);
    }
}