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
    // let mut hash = HashMap::new();
    // let s = String::from("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
    // hash.insert(s, 1);
    // println!("{:?}", hash);
    input! {
        t: usize
    }
    for i in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
        }
        let mut graph = vec![vec![]; n];
        for j in 0..n {
            input! {
                u_j: usize,
                v_j: usize,
            }
            graph[u_j-1].push(v_j-1);
            graph[v_j-1].push(u_j-1);
        }
    }
}

// fn dfs(graph: &mut Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, c: &Vec<usize>) {
//     seen[v] = true;
//     for i in 0..graph[v].len() {
//         let next_v = graph[v][i];
//         if seen[next_v] {continue}
//         dfs(graph, next_v, seen, c);
//     }
//     seen[v] = false;
// }

fn dfs(graph: &mut Vec<Vec<usize>>, vt: usize, va: usize, seen_t: &mut Vec<bool>, seen_a: &mut Vec<bool>, c: &Vec<usize>) {
    seen_t[vt] = true;
    seen_a[va] = true;
    
    for i in 0..graph[vt].len() {
        for j in 0..graph[va].len() {
            let vnt = graph[vt][i];
            let vna = graph[va][j];
            if seen_t[vnt] {continue}
            if seen_a[vna] {continue}
            if c[vnt] == c[vnc] {continue}
            dfs(graph, vnt, vna, seen_t, seen_a, c);
            
        }
    }

}