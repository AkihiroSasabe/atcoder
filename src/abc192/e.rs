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
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut t = vec![];
    let mut k = vec![];
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
            ti: usize,
            ki: usize,
        }
        a.push(ai);
        b.push(bi);
        t.push(ti);
        k.push(ki);
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        graph[a[i]].push((b[i], t[i], k[i]));
        graph[b[i]].push((a[i], t[i], k[i]));
    }

    let dists = get_minimum_distances_by_dijkstra_algorithm(&graph, x);
    if dists[y] == 1 << 60 {
        println!("-1");
    }
    else {
        println!("{}", dists[y]);
    }
}


// ダイクストラ法
fn get_minimum_distances_by_dijkstra_algorithm(graph: &Vec<Vec<(usize, usize, usize)>>, start_v: usize) -> Vec<usize> {
    // graph[v] := vec![(nv0, w0), (nv1, w1), ..., ]
    // 隣接頂点と、その重み
    
    use std::cmp::{Ordering, Reverse};
    // ヒープを使ったダイクストラ法
    // 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
    // 単純なダイクストラ法 計算量: O(|V|^2)
    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    //     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
    //     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース

    const INF: usize = 1 << 60; // usizeが取りうる値は0~2^64。
    let mut distances = vec![INF; graph.len()];
    distances[start_v] = 0;

    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
    let mut heap = std::collections::BinaryHeap::new();
    heap.push( Reverse((distances[start_v], start_v)));

    while !heap.is_empty() {
        let Reverse((dist, v)) = heap.pop().unwrap();
        
        // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
        if dist > distances[v] {continue}

        // v を始点とした辺の緩和
        for &(nv, weight, cycle) in graph[v].iter() {
            let n_dist = if distances[v] % cycle == 0 {
                distances[v] + weight
            } else {
                (distances[v] / cycle + 1) * cycle + weight
            };

            // 緩和できる場合
            if distances[nv] > n_dist {
                // 緩和
                distances[nv] = n_dist;
                // 到達可能で最短距離が未確定な頂点リストに追加
                heap.push( Reverse((distances[nv], nv)));
            }
        }
    }
    return distances
}
