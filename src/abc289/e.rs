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
    // 2023-10-14 11:21-13:02 (1h41m = 101m)
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
        for j in 0..m {
            input! {
                u_j: usize,
                v_j: usize,
            }
            graph[u_j-1].push(v_j-1);
            graph[v_j-1].push(u_j-1);
        }

        solve(n, &c, &graph);
    }
}

fn solve(n: usize, c: &Vec<usize>, graph: &Vec<Vec<usize>>) {
    if c[0] == c[n-1] {
        println!("-1");
        return
    }

    // BFSで(高橋, 青木) が (0, n-1) -> (n-1, 0) となる最短距離を求める
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut distance: HashMap<(usize, usize), u32> = HashMap::new();
    let init_state = (0_usize, n-1);
    distance.insert(init_state.clone(), 0);

    queue.push_back(init_state);
    while queue.len() > 0 {
        let state = queue.pop_front().unwrap();
        let v0 = state.0;
        let v1 = state.1;

        let next_distance = *distance.get(&state).unwrap() + 1;
        
        for &next_v0 in graph[v0].iter() {
            for &next_v1 in graph[v1].iter() {
                if c[next_v0] == c[next_v1] {continue}
                let next_state = (next_v0, next_v1);
                if distance.contains_key(&next_state) {continue}
                distance.insert(next_state.clone(), next_distance);
                queue.push_back(next_state);
            }
        }
    }
    let final_state = (n-1, 0);
    if !distance.contains_key(&final_state) {
        println!("-1");
    }
    else {
        let ans = *(distance.get(&final_state).unwrap());
        println!("{}", ans);
    }

}