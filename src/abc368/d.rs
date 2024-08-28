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
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
fn main() {
    // 2024-08-24 21:31-22:16 (45min)
    input! {
        n: usize,
        k: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input! {
            ai: usize,
            bi: usize,
        }
        graph[ai-1].push(bi-1);
        graph[bi-1].push(ai-1);
    }
    input!{
        vvv: [usize; k]
    }

    let mut keeps: HashSet<usize> = HashSet::new();

    let v0 = vvv[0] - 1;
    for vv in vvv {
        keeps.insert(vv-1);
    }

    let mut seen = vec![false; n];
    let mut subs = vec![0; n];
    dfs(v0, &graph, &mut seen, &mut subs, &keeps);
    // println!("subs = {:?}", subs);
    println!("{}", subs[v0]);



}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, subs: &mut Vec<usize>, keeps: &HashSet<usize>) -> bool {
    seen[v] = true;
    let mut is_remain = false;
    // println!("v = {v} -------------");

    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        let flag =  dfs(next_v, graph, seen, subs, keeps);
        if flag {
            is_remain = true;
            subs[v] += subs[next_v];
        }
        // println!("subs[nv={next_v}] = {:?}", subs[next_v]);
    }

    if keeps.contains(&v) {
        is_remain = true;
    }

    if is_remain {
        subs[v] += 1;
    }

    // println!("v={}, subs[v] = {}, is_remain = {:?}", v, subs[v], is_remain);

    return is_remain

}