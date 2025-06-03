#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::{cmp::{max, min, Ordering, Reverse}, hash::Hash, vec};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {

    // 2025-05-31 21:35-22:40 (1h5min)
    // 2025-05-31 22:40-01:28 (2h48min)
    // 2025-06-01 08:41-09:40 (59min)
    // Total: 4h52min

    // println!("{}", 0_u64.leading_zeros());
    // println!("{}", 2_u64.leading_zeros());
    // return;
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
            wi: usize,
        }
        edges.push((ui, vi, wi));
        edges.push((vi, ui, wi));
        graph[ui].push((vi, wi));
        graph[vi].push((ui, wi));
    }

    let mut exist_edge = vec![true; 2*m];
    // 大きなビットの桁が貪欲に最小にするように、グラフの辺を枝刈りしていく。
    // O(bit数 * (N + M))
    let num_bits = 30;
    let mut ans = 0;
    for b in (0..num_bits).rev() {
        let init = 1 << b;
        let mut dists: Vec<usize> = vec![0; n];
        for i in 0..n {
            dists[i] |= init;
        }
        dists[0] = 0;
        // println!("b = {:03b}, init dists = {:?} ---- ----", (1 << b), dists);
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while queue.len() > 0 {
            let v = queue.pop_front().unwrap();
            for &(nv, nw) in &graph[v] {
                let nd = (1 << b) & nw;
                if dists[nv] > nd {
                    dists[nv] = nd;
                    queue.push_back(nv);
                }
            }
        }
        // println!("dists = {:?}", dists);

        // 使わないエッジは消していく
        let mut g2 = vec![vec![]; n];
        for i in 0..2*m {
            if !exist_edge[i] {continue}
            let (ui, vi, wi) = edges[i];

            if (dists[n-1] & (1 << b)) == 0 {
                if wi & (1 << b) != 0 {
                    // 頂点n-1の OR が 0 なら 重みが 0 じゃないエッジは消す。
                    exist_edge[i] = false;
                    continue;
                }
                // この条件↓はなくてもいけるかも? (でも、1ケースだけ、TLEするので、追加)
                else {
                    // 重みが 0 のエッジでも, ORが 0 じゃない頂点は到達不可能なので、エッジを消す。
                    if (dists[ui] & (1 << b) != 0) || (dists[vi] & (1 << b) != 0) {
                        exist_edge[i] = false;
                        continue;
                    } 
                }
            }
            g2[ui].push((vi, wi));
        }

        if (dists[n-1] >> b) != 0 {
            ans |= 1 << b;
        }

        graph = g2;
        // println!("graph = {:?}", graph);
    }

    // let mut seen = vec![false; n];
    // fn dfs(v: usize, graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<bool>, or: usize) -> usize {
    //     let n = graph.len();
    //     if v == n - 1 {
    //         return or;
    //     }

    //     seen[v] = true;
    //     let inf = 1 << 60;
    //     for &(nv, nw) in &graph[v] {
    //         if seen[nv] {continue}
    //         let n_or = or | nw;
    //         let ans = dfs(nv, graph, seen, n_or);
    //         if ans == inf {continue}
    //         return ans;
    //     }
    //     // ゴールのn-1に到達できない場合
    //     return inf
    // }
    // let ans = dfs(0, &graph, &mut seen, 0);

    println!("{}", ans);
}
