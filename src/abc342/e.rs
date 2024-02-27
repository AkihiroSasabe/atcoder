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
    // 2024-02-26 20:05-21:29 (1h24min)
    // 2024-02-27 12:18-12:42 (24min)
    // total: 1h48min
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut ldkciab = vec![];
    for i in 0..m {
        input!{
            li: isize, // start時刻
            di: isize, // 公差
            ki: isize, // 項数
            ci: isize, // つぎの駅えの所要時間
            ai: usize,
            bi: usize,
        }
        let ui = ai - 1;
        let vi = bi - 1;
        let from_last_i = li + (ki-1)*di;
        let to_last_i = from_last_i + ci;
        ldkciab.push((i, li, di, ki-1, ci, ui, vi, from_last_i, to_last_i));
        graph[vi].push((i, li, di, ki-1, ci, ui, vi, from_last_i, to_last_i));
    }

    // 一番遅い時間
    let mut distance = vec![-1; graph.len()];
    // 辺を格納
    let mut heap = std::collections::BinaryHeap::new();

    for &edge in graph[n-1].iter() {
        let i = edge.0;
        let si = edge.1;
        let di = edge.2;
        let ki = edge.3;
        let ci = edge.4;
        let ui = edge.5;
        let vi = edge.6;
        let from_last_i = edge.7;
        let to_last_i = edge.8;

        distance[ui] = max(distance[ui], si + di * ki);
        heap.push((distance[ui], i));
    }
    while !heap.is_empty() {
        let (dist_u, i) = heap.pop().unwrap();

        let si = ldkciab[i].1;
        let di = ldkciab[i].2;
        let ki = ldkciab[i].3;
        let ci = ldkciab[i].4;
        let ui = ldkciab[i].5; // つぎ
        let vi = ldkciab[i].6; // 緩和済み
        let from_last_i = ldkciab[i].7;
        let to_last_i = ldkciab[i].8;

        // ゴミはスキップ
        if distance[ui] > dist_u {continue}

        // 頂点uiを始点とした辺の緩和
        for &edge in graph[ui].iter() {
            let i = edge.0;
            let si = edge.1;
            let di = edge.2;
            let ki = edge.3;
            let ci = edge.4;
            let ui = edge.5;
            let vi = edge.6;
            let from_last_i = edge.7;
            let to_last_i = edge.8;
            // 到達可能か?
            if si <= dist_u - ci {
                let num = (dist_u - ci - si) / di;
                let departure_time = if num <= ki {
                    si + num * di
                } 
                else {
                    si + ki * di
                };
                // 緩和可能か?
                if distance[ui] < departure_time {
                    distance[ui] = departure_time;
                    heap.push((departure_time, i));
                }
            }        
        }
    }

    // o -> o

    for i in 0..n-1 {
        if distance[i] != -1 {
            println!("{}", distance[i]);
        }
        else {
            println!("Unreachable");
        }
    }


}


// ダイクストラ法
mod dijkstra_algorithm {
    use std::cmp::Ordering;
    // Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
    // 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
    #[derive(Debug, Clone)]
    pub struct Edge {
        pub neighbor: usize,
        pub weight: usize,
    }
    impl Edge {
        fn new(neighbor: usize, weight: usize) -> Self {
            return Edge {neighbor, weight}
        }
    }
    pub fn get_minimum_distance(graph: &Vec<Vec<Edge>>, start_v: usize) -> Vec<usize> {
        // ヒープを使ったダイクストラ法
        // 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
        // 単純なダイクストラ法 計算量: O(|V|^2)
        // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
        //     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
        //     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース

        const INF: usize = 1 << 60; // usizeが取りうる値は0~2^64。
        let mut distance = vec![INF; graph.len()];
        distance[start_v] = 0;

        // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
        // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(State {cost: distance[start_v], position: start_v});
        while !heap.is_empty() {
            let state = heap.pop().unwrap();
            let min_v = state.position;
            let min_dist = state.cost;

            // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
            if min_dist > distance[min_v] {continue}

            // min_vを始点とした辺の緩和
            for edge in graph[min_v].iter() {
                // 緩和できる場合
                if distance[edge.neighbor] > distance[min_v] + edge.weight {
                    // 緩和
                    distance[edge.neighbor] = distance[min_v] + edge.weight;
                    // 到達可能で最短距離が未確定な頂点リストに追加
                    heap.push( State {cost: distance[edge.neighbor], position: edge.neighbor});
                }
            }
        }

        return distance
    }

    pub fn convert_graph(graph: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<Edge>> {
        let mut new_graph = vec![vec![]; graph.len()];

        for v0 in 0..graph.len() {
            for edge in graph[v0].iter() {
                let v1 = edge[0];
                let weight = edge[1];
                new_graph[v0].push(Edge::new(v1, weight));
            }
        }
        return new_graph
    }

    // BinaryHeapの根を最大値ではなく最小値にするために構造体を書き換える
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: usize,
    }

    // The priority queue depends on `Ord`.
    // Explicitly implement the trait so the queue becomes a min-heap
    // instead of a max-heap.
    // impl トレイト名 for 構造体名
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    // `PartialOrd` needs to be implemented as well.
    // impl トレイト名 for 構造体名
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}