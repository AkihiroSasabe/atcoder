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
fn main() {
    // 2023-12-13 21:08-21:40 (32min)
    input! {
        n: usize,
        q: usize,
        mut x: [usize; n],
        ab: [(usize, usize); n-1],
        vk: [(usize, usize); q]
    }
    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    let mut heaps: Vec<BinaryHeap<(usize, usize)>> = vec![BinaryHeap::new(); n];
    for i in 0..n-1 {
        let ui = ab[i].0 - 1;
        let vi = ab[i].1 - 1;
        graph[ui].push(vi);
        graph[vi].push(ui);
        degrees[ui] += 1;
        degrees[vi] += 1;
    }
    let mut seen = vec![false; n];
    dfs(0, &graph, &mut seen, &degrees, &mut heaps, &x);
    // println!("heaps = {:?}", heaps);

    // 各頂点に最大長さ20のヒープを持たせる方針で。
    // 帰りがけのDFSで良いかな。
    for i in 0..q {
        let v = vk[i].0 - 1;
        let k = vk[i].1;
        
        let mut temp = vec![];
        for j in 0..k {
            let (xx, vv) = heaps[v].pop().unwrap();
            temp.push((xx, vv));
        }
        println!("{}", temp[k-1].0);
        for j in 0..k {
            heaps[v].push(temp[j]);
        }
    }

}


fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, degrees: &Vec<usize>, heaps: &mut Vec<BinaryHeap<(usize, usize)>>, x: &Vec<usize>) {
    seen[v] = true;
    heaps[v].push((x[v], v));
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, degrees, heaps, x);

        let mut temp = vec![];
        for _ in 0..20 {
            if let Some((xx, vv)) = heaps[next_v].pop() {
                heaps[v].push((xx, vv));
                temp.push((xx, vv));
            }
            else {
                break
            }
        }
        for j in 0..temp.len() {
            heaps[next_v].push(temp[j]);
        }
    }
    // // 葉
    // if degrees[v] == 1 && v != 0 {
    //     heaps[v].push((x[v], v));
    // }
}