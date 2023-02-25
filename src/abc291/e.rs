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
    input! {
        n: usize,
        m: usize,
    }
    // let mut x = vec![];
    // let mut y = vec![];
    let mut graph = vec![vec![]; n];
    let mut g2= vec![HashMap::new(); n];
    for i in 0..m {
        input! {
            x_i: usize,
            y_i: usize,
        }
        if !g2[x_i-1].contains_key(&(y_i-1)) {
            graph[x_i-1].push(y_i-1);
            g2[x_i-1].insert(y_i-1, 0);
        }
    }

    // トポロジカルソート
    let mut topological_sorted_list = vec![];
    let mut seen = vec![false; n];
    for v in 0..n {
        if seen[v] {continue}
        dfs(&graph, v, &mut seen, &mut topological_sorted_list);
    }
    topological_sorted_list.reverse();
    // println!("topological_sorted_list: {:?}", topological_sorted_list);

    // 答え
    let mut seen = vec![false; n];
    let mut max_depth = 0;
    let mut ans = vec![];
    let mut depth_list = vec![0; n];
    for i in 0..topological_sorted_list.len() {
        if depth_list[topological_sorted_list[i]] != 0 {continue}
        ans = vec![];
        get_depth(&graph, topological_sorted_list[i], &mut seen, 0, &mut max_depth, &mut ans, n, &mut depth_list);
        if ans.len() == n {break}
    }

    ans.reverse();
    // println!("ans: {:?}", ans);


    if ans.len() == n {
        println!("Yes");
        let mut aa = vec![0; n];
        for i in 0..ans.len() {
            aa[ans[i]] = i + 1;
        }
        for i in 0..n {
            print!("{} ", aa[i]);
        }
    }
    else {
        println!("No");
    }
    

}

fn dfs(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, sorted_list: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs(graph, *next_v, seen, sorted_list);
    }
    sorted_list.push(v);
}


fn get_depth(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, current_depth: usize, max_depth: &mut usize, ans: &mut Vec<usize>, n: usize, depth_list: &mut Vec<usize>) {
    seen[v] = true;
    depth_list[v] = max(*max_depth, current_depth);

    *max_depth = max(*max_depth, current_depth);
    if *max_depth == n-1 {
        ans.push(v);
        return
    }
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] && depth_list[next_v] <= depth_list[v] + 1  {continue}
        get_depth(graph, next_v, seen, current_depth + 1, max_depth, ans, n, depth_list);
        if *max_depth == n-1 {
            break
        }
    }
    if *max_depth == n-1 {
        ans.push(v);
        return
    }
}
