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
    // 2023-01-10 22:07-
    // 2023-01-14 23:46-0:56
    // 2023-01-15 16:51-
    // 2023-01-17 22:16-24:56
    // 3h以上   
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut graph = vec![vec![]; n];

    // 各頂点の入次数(入ってくる辺の本数)を記録していく (BFSのトポロジカルソートの為)
    let mut indegree = vec![0; n];
    // ユニークな頂点を格納していく (グラフの頂点数カウントの為)
    let mut unique = BTreeMap::new();

    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i-1);
        b.push(b_i-1);
        graph[a_i - 1].push(b_i - 1);

        // 入次数の更新
        indegree[b_i - 1] += 1;
        unique.insert(a_i-1, 1);
        unique.insert(b_i-1, 1);
    }

    // トポロジカルソート開始
    // 入次数が0の頂点の集合
    // トポロジカルソートでは、普通キューを使うがキューじゃなくてもいい。Heapでも何でもok。
    // let mut todo = VecDeque::new();
    let mut todo = BinaryHeap::new();
    for i in 0..n {
        if indegree[i] == 0 {
            // todo.push_back(*k);
            todo.push(-1 * i as isize);
        }
    }
    // println!("todo: {:?}", todo);

    let mut topological_sorted = vec![];
    while todo.len() != 0 {
        // let v = todo.pop_front().unwrap();
        let v = (-1 * todo.pop().unwrap()) as usize;
        topological_sorted.push(v);
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            indegree[next_v] -= 1;
            if indegree[next_v] == 0 {
                // todo.push_back(next_v);
                todo.push(-1 * next_v as isize);
            }
        }
    }

    let is_cycle = topological_sorted.len() != n;

    // println!("topological_sorted: {:?}", topological_sorted);
    // println!("unique: {:?}", unique);

    if is_cycle {
        println!("{}", -1);
    }
    else {
        for i in 0..topological_sorted.len() {
            print!("{} ", topological_sorted[i] + 1);
        }
    }
}

// 有向グラフが閉路か確認 (けんちょん方式。一般的ではないし、BFSトポロジカルソートの方が確実なのでオススメできない　https://qiita.com/drken/items/a803d4fc4a727e02f7ba#4-6-%E3%82%B5%E3%82%A4%E3%82%AF%E3%83%AB%E6%A4%9C%E5%87%BA )
fn detect_directed_cycle(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, finished: &mut Vec<bool>, is_cycle: &mut bool) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i];
        if seen[next_v] && !finished[v] {
            *is_cycle = true;
            return 
        }
        if seen[next_v] {continue}
        detect_directed_cycle(graph, next_v, seen, finished, is_cycle);
        if *is_cycle {return}
    }
    // 帰りがけの頂点
    finished[v] = true;
}

