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
    // 2023-12-16 18:00-19:15 (75min)
    // 2023-12-17 12:24-13:21 (57min)
    // total: 132min
    input! {
        n: usize,
        m: usize,
        h: [isize; n],
        uv: [(usize, usize); m]
    }

    // ダイクストラ法は負の辺に適応できない。
    // 標高差によって、楽しさが負になったり、正になったりするので、これを全部正になる量に置き換えることを考える。
    // 楽しさの代わりに、辛さの概念を導入して、辛さ := - 楽しさ としても負の辛さがでてしまう。
    // そこで、潜在的な辛さ := 辛さ - 標高差 という概念を導入すると、
    // ここで、山を下るとき、負の辛さになるが、潜在的な辛さは標高差で相殺して常に0になる。
    // 一方で、山を登るとき、正の辛さであり、標高差はこの絶対値より小さいので、潜在的な辛さは常に正になる。
    // よって潜在的な辛さをエッジとして、ダイクストラ法を適応すれば良い。

    // 辺の重みが、頂点の移動に伴う、潜在的な辛さの変化量と対応
    // 潜在的な辛さ := 現在の辛さ - 頂点0を基準とした標高
    // 例えば、(現在の辛さ, 頂点0を基準とした標高, 潜在的な辛さ) とすると、
    // 頂点0: (0, 0, 0)
    // 頂点1: (-2, -2, 0)
    // 頂点2: (4, 2, 2) or (6, 2, 4)
    // 頂点3: (-3, -5, 2)
    // となる。

    let mut graph: Vec<Vec<Vec<usize>>> = vec![vec![]; n];
    for i in 0..m {
        let u = uv[i].0 - 1;
        let v = uv[i].1 - 1;

        // u -> v に行く潜在的な辛さ
        let pain_uv: isize;
        let pain_vu: isize;
        if h[u] < h[v] {
            // v -> u へ降りるときは、楽しさ(+Δh) と 標高が低くなる(-Δh)　が、相殺するので、総合的な潜在的辛さ が 0 となる。
            pain_vu = 0;

            // u -> v へ登るときは、楽しさ -2Δh だけど、標高が高くなる (+Δh)
            pain_uv = h[v] - h[u];
        }
        else {
            pain_vu = h[u] - h[v];
            pain_uv = 0;
        }
        let pain_vu = pain_vu as usize;
        let pain_uv = pain_uv as usize;
        graph[u].push(vec![v, pain_uv]);
        graph[v].push(vec![u, pain_vu]);
    }

    let graph = dijkstra_algorithm::convert_graph(&graph);
    // for i in 0..n {
    //     println!("graph[{i}] = {:?}", graph[i]);
    // }
    
    // 辛さポテンシャル
    let dist = dijkstra_algorithm::get_minimum_distance(&graph, 0);
    // println!("dist = {:?}", dist);

    let mut ans = 0;
    for i in 0..n {
        // 辛さポテンシャル
        let potential_turasa: isize = dist[i] as isize;

        // 実際の辛さ
        let real_turasa: isize = potential_turasa + h[i] - h[0];
        // println!("real_turasa = {:?}", real_turasa);

        let fun: isize = - real_turasa;

        ans = max(ans, fun);
    }
    println!("{}", ans);

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