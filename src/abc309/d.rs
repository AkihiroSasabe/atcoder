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
        n1: usize,
        n2: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n1 + n2];
    let mut graph1 = vec![vec![]; n1];
    let mut graph2 = vec![vec![]; n1+n2];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        graph[a_i-1].push(b_i-1);
        graph[b_i-1].push(a_i-1);

        if a_i <= n1 {
            graph1[a_i-1].push(b_i-1);
            graph1[b_i-1].push(a_i-1);
        }
        else {
            graph2[a_i-1].push(b_i-1);
            graph2[b_i-1].push(a_i-1);
        }
    }
    let mut distance = vec![-1_isize; n1 + n2];
    distance[0] = 0;
    distance[n1+n2-1] = 0;

    let mut todo: VecDeque<usize> = VecDeque::new();
    todo.push_back(0);
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        for next_v in &graph1[v] {
            if distance[*next_v] != -1 {continue}
            distance[*next_v] = distance[v] + 1;
            todo.push_back(*next_v)
        }
    }

    let mut todo: VecDeque<usize> = VecDeque::new();
    todo.push_back(n1 + n2 - 1);
    while todo.len() != 0 {
        let v = todo.pop_front().unwrap();
        for next_v in &graph2[v] {
            if distance[*next_v] != -1 {continue}
            distance[*next_v] = distance[v] + 1;
            todo.push_back(*next_v)
        }
    }

    let mut max_1 = 0;
    for i in 0..n1 {
        max_1 = max(max_1, distance[i]);
    }

    let mut max_2 = 0;
    for i in n1..n1+n2 {
        max_2 = max(max_2, distance[i]);
    }
    println!("{}", max_1 + max_2 + 1);
    



}