#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-02 16:47-17:00 (13min)
    // 2025-02-02 18:26-19:06 (40min)
    // Total: 53min
    // 全体の計算量は、O(N^3 log N) = 頂点0の次数O(N) * (グラフの再構築O(N^2) + ダイクストラ法: O(N^2logN)) 
    // 頂点0に隣接するエッジを1本ずつ切ったグラフ、頂点0の次数の分だけ作ったが、
    // 
    // 公式解説では、頂点0に隣接するエッジを一気に全部まとめて切る。
    // その場合は、カットした後のグラフについて、ワーシャルフロイド法で全頂点間距離を求めておくのがよい。
    // 頂点0に隣接する頂点から、2個の頂点{v0, v1}を選んで(最大nC2通り)、dist[0, v0] + dist[0, v1] + dist[v0, v1] が最小になるやつを選べばよい。
    input! {
        n: usize,
        m: usize,
    }
    let mut edges_on_0 = vec![];
    let mut us = vec![];
    let mut vs = vec![];
    let mut ls = vec![];
    for i in 0..m {
        input!{
            usi: Usize1,
            vsi: Usize1,
            lsi: usize,
        }
        us.push(usi);
        vs.push(vsi);
        ls.push(lsi);
        if usi == 0 {
            // ※ usi < vsi
            edges_on_0.push((vsi, lsi, i));
        }
    }
    if edges_on_0.len() < 2 {
        println!("-1");
        return
    }
    const INF: usize = 1 << 60; // usizeが取りうる値は0~2^64。
    use dijkstra_algorithm::{Edge, get_minimum_distance, convert_graph};
    let mut ans = INF;
    for (vsi, lsi, ii) in edges_on_0 {
        // 0と繋がっているエッジを1本抜いたグラフを、頂点0の次数の数だけ作る。
        // その状態で、"抜いたエッジの重み" + "0と抜いた頂点の距離" の最小が答え。 
        // 全体の計算量は、O(N^3 log N) = 頂点0の次数O(N) * (グラフの再構築O(N^2) + ダイクストラ法: O(N^2logN)) 
        // N <= 300 なので、N^3 = 2.7 * 10^7, logN=(2log10 + log3)
        // ギリギリ間に合う計算

        // グラフの再構築 O(N+M) = O(N + N^2) = O(N^2)
        let mut graph = vec![vec![]; n];
        for i in 0..m {
            if i == ii {continue}
            let ui = us[i];
            let vi = vs[i];
            let wi = ls[i];

            graph[ui].push((vi, wi));
            graph[vi].push((ui, wi));
        }
        let graph = convert_graph(&graph);
        // ダイクストラ法: O(|V^2| logV)
        let dists = get_minimum_distance(&graph, 0);
        let dist = dists[vsi] + lsi;
        ans = min(ans, dist);
    }
    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
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

    pub fn convert_graph<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<Edge>> 
        where
        T: Copy + Into<usize>, // TがInto<usize>を実装していることを要求
    {
        let mut new_graph = vec![vec![]; graph.len()];

        for v in 0..graph.len() {
            for i in 0..graph[v].len() {
                let nv = graph[v][i].0;
                let weight = graph[v][i].1.into(); // `Into`トレイトのメソッドを使用;
                new_graph[v].push(Edge::new(nv, weight));
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