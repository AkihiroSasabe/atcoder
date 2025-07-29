#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n];
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
        }
        a.push(ai);
        b.push(bi);
        g[ai].push(bi);
        g[bi].push(ai);
    }

    let dists = bfs(0, &g);
    let mut ans = vec![0; n];
    let mut heap = BinaryHeap::new();
    for v in 1..n {
        heap.push(Reverse((dists[v], v)));
    }
    loop {
        if let Some(Reverse((dist, v))) =  heap.pop() {
            let mut is_ok = false;
            for &nv in g[v].iter() {
                if dists[nv] == dist - 1 {
                    ans[v] = nv + 1;
                    is_ok = true;
                    break;
                }
            }
            if !is_ok {
                println!("No");
                return;
            }
        }
        else {
            break
        }
    }
    println!("Yes");
    for i in 1..n {
        println!("{}", ans[i]);
    }


}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[v0] = 0;
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    return distance
}