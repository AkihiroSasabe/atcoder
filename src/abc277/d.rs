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
    let mut hash_map = HashMap::new();
    for i in 0..a.len() {
        if hash_map.contains_key(&a[i]) {
            *hash_map.get_mut(&a[i]).unwrap() += a[i];
        }
        else {
            hash_map.insert(a[i], a[i]);
        }
    }
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in &hash_map {
        if hash_map.contains_key(&((*k + 1) % m)) {
            if graph.contains_key(k) {
                (*graph.get_mut(k).unwrap()).push((*k + 1) % m);                
            }
            else {
                graph.insert(*k, vec![(*k + 1) % m]);
            }
        }
    }
    // println!("hash_map: {:?}", hash_map);
    // println!("graph: {:?}", graph);
    let mut max_sum = 0;
    for (k, v) in &hash_map {
        let mut seen = HashMap::new();
        dfs(*k, &mut graph, &hash_map, &mut seen, 0, &mut max_sum);
    }


    let a_sum: usize = a.iter().sum();
    println!("{}", a_sum - max_sum);


    
    // let mut tinko = HashMap::new();
    // for i in 0..n {
    //     if tinko.contains_key(&a[i]) {
    //         *tinko.get_mut(&a[i]).unwrap() += 1_usize;
    //     }
    //     else {
    //         tinko.insert(a[i], 1);
    //     }
    // }
    // let mut hash_map = HashMap::new();

    // for (k, v) in &tinko {
    //     if *k == (*k+1)  % m && tinko.contains_key(&((*k+1)  % m)) {
    //         hash_map.entry(*k).or_insert(vec![]).push((*k+1) % m);
    //         hash_map.entry((*k+1) % m).or_insert(vec![]).push(*k);
    //     }
    // }
    // for i in 0..n {
        
    // }
    // dfs(&hash_map, &mut ans, 0, 0, &mut seen);

    // let mut ans = 0;

}


fn dfs(v: usize, graph: &mut HashMap<usize, Vec<usize>>, hash_map: &HashMap<usize, usize>, seen: &mut HashMap<usize, bool>, sum: usize, max_sum: &mut usize) {
    seen.insert(v, true);
    let sum = sum + hash_map[&v];
    *max_sum = max(*max_sum, sum);
    if graph.contains_key(&v) {
        for i in 0..graph[&v].len() {
            let next_v = graph[&v][i];
            if seen.contains_key(&next_v) {continue}
            dfs(next_v, graph, hash_map, seen, sum, max_sum);
        }
    }
}


// fn dfs(graph: &HashMap<usize, Vec<usize>>, ans: &mut usize, v: usize, parent: usize, seen: &mut HashMap<usize, bool>) {
//     seen.insert(v, true);
//     if !graph.contains_key(&v) {
//         return
//     }
//     for i in 0..graph[&v].len() {
//         let v_next = graph[&v][i];
//         if v_next == parent {continue}
//         if seen.contains_key(&v_next) {continue}
//         if *ans < v_next {
//             *ans = v_next;
//         }
//         dfs(graph, ans, v_next, v, seen);
//     }
// }
