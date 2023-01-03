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
    // 2023-01-04 水曜日 
    // 19:55-19:59 (4min)
    // 20:47-22:31 (1h44m)
    // 2023-01-05 木曜日
    // 12:36-14:51 (2h15m)
    // total: 4h3m
    input! {
        n: usize,
    }

    let mut aaa = vec![vec![]; 2*n-1];
    let mut target = 0;
    for i in 0..2*n-1 {
        target += 1;
        for j in 0..(2*n-1-i) {
            input! {
                a_ij: usize
            }
            aaa[i].push(vec![j+target, a_ij]);
        }
        // println!("{:?}", aaa[i]);
    }

    // let mut INF = 1 << 60;
    // let mut graph = vec![vec![INF; 2*n]; 2*n];
    // let mut source = 0;
    // let mut target = 1;
    // let mut a = vec![];
    // for i in 0..2*n-1 {
    //     input! {
    //         a_i: [usize; 2*n-1-i]
    //     }
    //     // a.push(a_i);
    //     for j in 0..a_i.len() {
    //         graph[i][j+target] = a_i[j];
    //         graph[j+target][i] = a_i[j];
    //     }
    //     target += 1;
    // }

    // let mut hash = BTreeMap::new();
    // hash.insert(1,100);
    // hash.insert(2,100);
    // let unk = hash.iter().next();
    // println!("unk: {:?}", unk);
    let mut unused = BTreeMap::new();
    for i in 0..2*n {
        unused.insert(i, 1);
    }
    // let mut seen = vec![false; 2*n];
    let mut fun = 0;
    // dfs(0, &mut seen, &graph, &mut fun, 0, 1);
    // let mut unused = vec![];
    // for i in 0..2*n {
    //     unused.push(2*n - 1 - i);
    // }
    dfs(&aaa, &mut unused , &mut fun, 0, 1);
    println!("{}", fun);

    // for i in 0..2*n {
    //     println!("{:?}", graph[i]);
    // }

    // 組み合わせの総数は、
    // v-1 * v-3 * v-5 * ... * 1
    // v = 2n <= 16より
    // 2_027_025 ~ 2*10^6
}

fn dfs(graph: &Vec<Vec<Vec<usize>>>, unused: &mut BTreeMap<usize, usize>, fun: &mut usize, fun_temp: usize, depth: usize) {
    let (&source, _) = unused.iter().next().unwrap();
    unused.remove(&source);
    for target in 0..graph[source].len() {
        let next_v = graph[source][target][0];
        if !unused.contains_key(&next_v) {continue}
        unused.remove(&next_v);
        let next_weight = graph[source][target][1];
        let next_fun_temp = fun_temp ^ next_weight;
        // println!("source: {source}, target: {next_v}");
        if depth == (graph.len()+1) / 2 {
            // println!("next_fun_temp: {}, depth: {}", next_fun_temp, depth);
            *fun = max(*fun, next_fun_temp);
        }
        else {
            dfs(graph, unused, fun, next_fun_temp, depth+1);
        }
        unused.insert(next_v, 1);
    }
    unused.insert(source, 1);
}


// fn dfs(v: usize, seen: &mut Vec<bool>, graph: &Vec<Vec<usize>>, fun: &mut usize, fun_temp: usize, depth: usize) {
//     // println!("v: {}, depth: {}", v, depth);
//     seen[v] = true;
    
//     for aikata in 0..graph.len() {
//         if seen[aikata] {continue}
//         seen[aikata] = true;
//         let next_fun_temp = fun_temp ^ graph[v][aikata];
//         println!("depth: {}, v: {}, aikata: {}, fun_temp: {fun_temp}, aisho:{}, next_fun_temp: {}", depth, v, aikata, graph[v][aikata], next_fun_temp);
//         if depth == graph.len() / 2 {
//             println!("next_fun_temp: {}, depth: {}", next_fun_temp, depth);
//             *fun = max(*fun, next_fun_temp);
//         }
//         for next_v in 0..graph.len() {
//             if seen[next_v] {continue}
//             dfs(next_v, seen, graph, fun, next_fun_temp, depth + 1);
//         }
//         seen[aikata] = false;
//     }
//     seen[v] = false;
// }
