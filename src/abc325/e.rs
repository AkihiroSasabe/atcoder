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
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: [[usize; n]; n]
    }

    // 電車だけで行く場合の最短経路
    let mut car = vec![vec![]; n];
    let mut train = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            car[i].push(Edge {neighbor: j, weight: d[i][j] * a});
            train[i].push(Edge {neighbor: j, weight: d[i][j] * b + c});
        }
    }
    let distance_list_from_train: Vec<usize> = get_minimum_distance(&train, n, n-1);
    let distance_list_from_car: Vec<usize> = get_minimum_distance(&car, n, 0);

    let mut ans: usize = 1_000_000_000_000_000_000;
    for i in 0..n {
        let dist = distance_list_from_car[i] + distance_list_from_train[i];
        ans = min(ans, dist);
    }
    println!("{}", ans);

    // bfs(0, )

    // 社用車だけで行く場合の最短経路


}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[v0] = 0;
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    return distance
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

// Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
// 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
#[derive(Debug, Clone)]
struct Edge {
    neighbor: usize,
    weight: usize,
}

fn get_minimum_distance(graph: &Vec<Vec<Edge>>, v_num: usize, start_v: usize) -> Vec<usize> {
    // ヒープを使ったダイクストラ法
    // 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
    // 単純なダイクストラ法 計算量: O(|V|^2)
    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    //     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
    //     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース

    let INF = 1 << 60; // usizeが取りうる値は0~2^64。
    let mut distance = vec![INF; v_num];
    distance[start_v] = 0;

    // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
    // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
    let mut heap = BinaryHeap::new();
    heap.push(State {cost: distance[start_v], position: start_v});
    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let mut min_v = state.position;
        let mut min_dist = state.cost;

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