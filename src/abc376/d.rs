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
use rand::Rng;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    use dijkstra_algorithm::{Edge, get_minimum_distance};

    let mut graph = vec![vec![]; n];
    let mut reverse_graph = vec![vec![]; n];
    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        reverse_graph[vi - 1].push(ui - 1);

        g[ui - 1].push(Edge {neighbor: vi - 1, weight: 1});
        rg[vi - 1].push(Edge {neighbor: ui - 1, weight: 1});
    }

    let scc_list = decompositon_of_strongly_connected_components(&graph, &reverse_graph, n);

    // println!("scc_list = {:?}", scc_list);


    let dists = get_minimum_distance(&g, 0);
    let rdists = get_minimum_distance(&rg, 0);

    for i in 0..scc_list.len() {
        for j in 0..scc_list[i].len() {
            let v = scc_list[i][j];
            if v == 0 {
                if scc_list[i].len() == 1 {
                    println!("-1");
                    return;
                }
                else {
                    let mut ans = 1 << 60;
                    for k in 0..scc_list[i].len() {
                        let nv = scc_list[i][k];
                        if v == nv {continue}
                        ans = min(ans, dists[nv] + rdists[nv]);
                    }
                    println!("{}", ans);
                    return;
                }
            }
        }
    }















}



// 1回目のDFS
fn dfs1(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, sorted_list: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs1(graph, *next_v, seen, sorted_list);
    }
    sorted_list.push(v);
}

// 2回目のDFS。トポロジカルソートした番号の逆順から攻める。
fn dfs2(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, scc: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in graph[v].iter() {
        if seen[*next_v] {continue}
        dfs2(graph, *next_v, seen, scc);
    }
    scc.push(v);
}


// 強連結成分分解 (蟻本p285~p288) 計算量O(E)
fn decompositon_of_strongly_connected_components(graph: &Vec<Vec<usize>>, reverse_graph: &Vec<Vec<usize>>, v_num: usize) -> Vec<Vec<usize>>{

    // 1回目のDFS: トポロジカルソートする
    let mut reverse_topological_sorted_list = vec![];
    let mut seen = vec![false; v_num];
    for v in 0..v_num {
        if seen[v] {continue}
        dfs1(graph, v, &mut seen, &mut reverse_topological_sorted_list);
    }

    // 2回目のDFS: グラフの辺を逆向きにして、たどり着ける頂点を強連結成分としてまとめる
    let mut scc_list = vec![];
    let mut seen = vec![false; v_num];
    reverse_topological_sorted_list.reverse();
    let topological_sorted_list = reverse_topological_sorted_list;
    for v in topological_sorted_list {
        if seen[v] {continue}
        let mut strongly_connected_components = vec![];
        dfs2(&reverse_graph, v, &mut seen, &mut strongly_connected_components);
        scc_list.push(strongly_connected_components);
    }

    return scc_list
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