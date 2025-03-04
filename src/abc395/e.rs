#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: isize,
    }
    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        rev_graph[vi-1].push(ui-1);
    }
    let dist = bfs(0, &graph, &rev_graph, x);
    // println!("dist = {:?}", dist);
    println!("{}", min(dist[n-1][0], dist[n-1][1]));
}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>, rev_graph: &Vec<Vec<usize>>, x: isize) -> Vec<Vec<isize>> {
    // v0が始点
    let init_distance = 1_000_000_000_000_000_000; // 10^18
    let mut heap: BinaryHeap<(isize, usize, usize)> = BinaryHeap::new();
    let mut distance = vec![vec![init_distance; 2]; graph.len()];
    distance[v0][0] = 0;
    heap.push((-0, v0, 0)); // 0がforwad, 1がrev状態

    while heap.len() > 0 {
        let (m_dist, v, state) = heap.pop().unwrap();
        // if (- m_dist) >= distance[v][state] {continue}

        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            let mut cost = distance[v][state] + 1;
            if state == 1 {
                cost += x;
            }
            if distance[nv][0] > cost {
                distance[nv][0] = cost;
                // queue.push_back(nv);
                heap.push((-cost, nv, 0));
            }
        }

        for i in 0..rev_graph[v].len() {
            let nv = rev_graph[v][i];
            let mut cost = distance[v][state] + 1;
            if state == 0 {
                cost += x;
            }
            if distance[nv][1] > cost {
                distance[nv][1] = cost;
                heap.push((-cost, nv, 1));
            }
        }
    }
    return distance
}