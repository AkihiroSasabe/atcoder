use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use proconio::marker::Chars;
fn main() {
    // 2023-12-06 15:00-16:03 (63min)

    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]    
    }
    let mut graph = vec![vec![]; n];
    let mut edges = vec![HashMap::new(); n];
    for i in 0..m {
        let v0 = abc[i].0 - 1;
        let v1 = abc[i].1 - 1;
        let c = abc[i].2;
        edges[v0].insert(v1, i+1);
        edges[v1].insert(v0, i+1);
        // graph[v0].push(vec![v1, c]);
        // graph[v1].push(vec![v0, c]);

        let edge = Edge{
            neighbor: v1,
            weight: c
        };
        graph[v0].push(edge);
        
        let edge = Edge{
            neighbor: v0,
            weight: c
        };
        graph[v1].push(edge);
    }

    let (dist_list, nokosu) = get_minimum_distance(&graph, n, 0);
    
    for i in 0..n {
        for &to in nokosu[i].iter() {
            if i == 0 && to == 0 {continue}
            let edge_ind = edges[i].get(&to).unwrap();
            print!("{} ", edge_ind);
        }
    }

}


// BinaryHeapの根を最大値ではなく最小値にするために構造体を書き換える
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    from: usize
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

fn get_minimum_distance(graph: &Vec<Vec<Edge>>, v_num: usize, start_v: usize) -> (Vec<usize>, Vec<HashSet<usize>>) {
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
    heap.push(State {cost: distance[start_v], position: start_v, from: start_v});
    let mut nokosu = vec![HashSet::new(); v_num];
    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let mut min_v = state.position;
        let mut min_dist = state.cost;
        let from = state.from;

        // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
        if min_dist > distance[min_v] {continue}
        nokosu[from].insert(min_v);

        // min_vを始点とした辺の緩和
        for edge in graph[min_v].iter() {
            // 緩和できる場合
            if distance[edge.neighbor] > distance[min_v] + edge.weight {
                // 緩和
                distance[edge.neighbor] = distance[min_v] + edge.weight;
                // 到達可能で最短距離が未確定な頂点リストに追加
                heap.push( State {cost: distance[edge.neighbor], position: edge.neighbor, from: min_v});
            }
        }
    }
    return (distance, nokosu)
}