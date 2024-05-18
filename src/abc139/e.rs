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
    // 2024-05-17 20:39-21:48 (1h9min)
    input! {
        n: usize,
        mut a: [[usize; n-1]; n]
    }
    for i in 0..n {
        for j in 0..n-1 {
            a[i][j] -= 1;
        }
    }

    let mut map = HashMap::new();
    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            map.insert((i,j), count);
            count += 1;
        }
    }

    let mut graph = vec![vec![]; n*(n-1)/2];
    let mut in_degrees = vec![0; n*(n-1)/2];
    let INF = 1 << 60;
    let mut days = vec![INF; n*(n-1)/2];

    for i in 0..n {
        let pre = (min(i, a[i][0]), max(i, a[i][0]));
        let mut v_pre: usize = *map.get(&pre).unwrap();
        for j in 1..n-1 {
            let now = (min(i, a[i][j]), max(i, a[i][j]));
            let v_now = *map.get(&now).unwrap();
            in_degrees[v_now] += 1;
            graph[v_pre].push(v_now);
            v_pre = v_now;
        }
    }
    let mut sorted_list: Vec<usize> = topological_sort(&graph, &mut in_degrees, &mut days);
    // println!("map = {:?}", map);
    // println!("graph = {:?}", graph);
    // println!("in_degrees = {:?}", in_degrees);
    // println!("sorted_list = {:?}", sorted_list);

    // 21:31-
    if sorted_list.len() != n * (n-1) / 2 {
        println!("-1");
    }
    else {
        let mut ans = 0;
        for i in 0..n*(n-1)/2 {
            ans = max(ans, days[i]);
        }
        println!("{}", ans);
    }
    



}

// トポロジカルソート(入次数が0のものからリストに入れていく)
fn topological_sort(graph: &Vec<Vec<usize>>, in_degrees: &mut Vec<usize>, days: &mut Vec<usize>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut sorted_list = vec![];
    for i in 0..graph.len() {
        if in_degrees[i] == 0 {
            queue.push_back(i);
            sorted_list.push(i);
            days[i] = 1;
        }
    }
    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        let day = days[v];
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            in_degrees[next_v] -= 1;
            // 入次数が0
            if in_degrees[next_v] == 0 {
                days[next_v] = min(days[next_v], day + 1);
                queue.push_back(next_v);
                sorted_list.push(next_v);
            }
        }
    }
    return sorted_list
}