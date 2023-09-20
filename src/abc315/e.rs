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
    // 2023-09-19 tue. 21:11-21:54 (43min)
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut graph_from1 = vec![vec![]; n];
    let mut in_degrees = vec![0; n];
    for i in 0..n {
        input!{ 
            c_i: usize,
            p_i: [usize; c_i],
        }
        for j in 0..c_i {
            graph[p_i[j] - 1].push(i);
            graph_from1[i].push(p_i[j] - 1);
            in_degrees[i] += 1;
        }
    }

    // let a = topological_sort(&graph, &mut in_degrees.clone());
    // println!("aa = {:?}", a);

    // DFSで読む必要のある本を記録
    let mut seen = vec![false; n];
    dfs(0, &graph_from1, &mut seen);

    // BFSによるトポロジカルソート (入次数が0のものからQueueに入れていく)
    let mut queue = VecDeque::new();
    let mut sorted_list = vec![];
    for i in 0..n {
        if in_degrees[i] == 0 && seen[i] {
            queue.push_back(i);
            sorted_list.push(i);
        }
    }
    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            in_degrees[next_v] -= 1;
            // 入次数
            if in_degrees[next_v] == 0 && seen[next_v] {
                queue.push_back(next_v);
                sorted_list.push(next_v);
            }
        }
    }

    // println!("sorted_list = {:?}", sorted_list);
    for i in 0..sorted_list.len() {
        if sorted_list[i] == 0 {continue}
        print!("{} ", sorted_list[i] + 1);
    }


}


// シンプルなDFS
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] {continue}
        dfs(next_v, graph, seen);
    }
}

// トポロジカルソート(入次数が0のものからリストに入れていく)
fn topological_sort(graph: &Vec<Vec<usize>>, in_degrees: &mut Vec<usize>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut sorted_list = vec![];
    for i in 0..graph.len() {
        if in_degrees[i] == 0 {
            queue.push_back(i);
            sorted_list.push(i);
        }
    }
    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            in_degrees[next_v] -= 1;
            // 入次数が
            if in_degrees[next_v] == 0 {
                queue.push_back(next_v);
                sorted_list.push(next_v);
            }
        }
    }
    return sorted_list
}