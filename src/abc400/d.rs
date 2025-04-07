#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        ys: Usize1,
        xs: Usize1,
        yt: Usize1,
        xt: Usize1,
    }
    // use dijkstra_algorithm::{Edge, get_minimum_distance, convert_graph};
    // let mut graph = vec![vec![]; n];
    // let converted_graph = convert_graph(&graph);
    // let distance_list_from_1 = get_minimum_distance(&converted_graph, 0);

    let ans = bfs(ys, xs, yt, xt, &s);
    println!("{}", ans);


}

fn bfs(ys: usize, xs: usize, yt: usize, xt: usize, s: &Vec<Vec<char>>) -> usize {
    // v0が始点
    let h = s.len();
    let w = s[0].len();
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    // let mut queue = VecDeque::new();
    let mut heap = BinaryHeap::new();
    let mut distance = vec![vec![init_distance; w]; h];
    distance[ys][xs] = 0;
    heap.push(Reverse((0, ys, xs)));
    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    

    while heap.len() > 0 {
        let Reverse((dist, y, x)) = heap.pop().unwrap();

        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] == '#' {
                let ndist = dist + 1;
                if distance[ny][nx] > ndist {
                    distance[ny][nx] = ndist;
                    heap.push(Reverse((ndist, ny, nx)));
                }

                let ny = 2*dir_y[i] + y as isize;
                let nx = 2*dir_x[i] + x as isize;
                if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
                let ny = ny as usize;
                let nx = nx as usize;
                if s[ny][nx] == '#' {
                    let ndist = dist + 1;
                    if distance[ny][nx] > ndist {
                        distance[ny][nx] = ndist;
                        heap.push(Reverse((ndist, ny, nx)));
                    }
                }
            }
            else {
                let ndist = dist;
                if distance[ny][nx] > ndist {
                    distance[ny][nx] = ndist;
                    heap.push(Reverse((ndist, ny, nx)));
                }
            }
        }
        
        // for i in 0..graph[v].len() {
        //     let nv = graph[v][i];
        //     if distance[nv] != init_distance {continue}
        //     distance[nv] = distance[v] + 1;
        //     queue.push_back(nv);
        // }
    }
    return distance[yt][xt]
}


// // ダイクストラ法
// mod dijkstra_algorithm {
//     use std::cmp::Ordering;
//     // Derive注釈は、自作の構造体に有用な振る舞いを追加する。(Debugはprintの為、Cloneはベクトルの要素として使う為に追加した)
//     // 参考: https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html?highlight=derive#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E5%B0%8E%E5%87%BA%E3%81%A7%E6%9C%89%E7%94%A8%E3%81%AA%E6%A9%9F%E8%83%BD%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
//     #[derive(Debug, Clone)]
//     pub struct Edge {
//         pub neighbor: usize,
//         pub weight: usize,
//     }
//     impl Edge {
//         fn new(neighbor: usize, weight: usize) -> Self {
//             return Edge {neighbor, weight}
//         }
//     }
//     pub fn get_minimum_distance(graph: &Vec<Vec<Edge>>, start_v: usize) -> Vec<usize> {
//         // ヒープを使ったダイクストラ法
//         // 密グラフではなく、疎グラフっぽいので、ヒープを利用したダイクストラ法で解く必要がある
//         // 単純なダイクストラ法 計算量: O(|V|^2)
//         // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
//         //     密グラフ|E| = |V|^2なら、O(|V|^2|log|V|)
//         //     疎グラフ|E| = |V|なら、O(|V|log|V|)          ←今回の問題のケース

//         const INF: usize = 1 << 60; // usizeが取りうる値は0~2^64。
//         let mut distance = vec![INF; graph.len()];
//         distance[start_v] = 0;

//         // ヒープを使ったダイクストラ法 計算量: O(|E|log|V|)
//         // ヒープの中には、到達可能な中で最短距離が未確定な頂点の、頂点番号と距離を格納
//         let mut heap = std::collections::BinaryHeap::new();
//         heap.push(State {cost: distance[start_v], position: start_v});
//         while !heap.is_empty() {
//             let state = heap.pop().unwrap();
//             let min_v = state.position;
//             let min_dist = state.cost;

//             // ゴミであるときはリトライ (ヒープの中には、同じ頂点vでも、更新前のd'[v]と更新後のd''[v]が格納されてしまう。ヒープのキー値d[v]を更新する代わりに、更新したd*[v]を挿入し続けるため)
//             if min_dist > distance[min_v] {continue}

//             // min_vを始点とした辺の緩和
//             for edge in graph[min_v].iter() {
//                 // 緩和できる場合
//                 if distance[edge.neighbor] > distance[min_v] + edge.weight {
//                     // 緩和
//                     distance[edge.neighbor] = distance[min_v] + edge.weight;
//                     // 到達可能で最短距離が未確定な頂点リストに追加
//                     heap.push( State {cost: distance[edge.neighbor], position: edge.neighbor});
//                 }
//             }
//         }

//         return distance
//     }

//     pub fn convert_graph<T>(graph: &Vec<Vec<(usize, T)>>) -> Vec<Vec<Edge>> 
//         where
//         T: Copy + Into<usize>, // TがInto<usize>を実装していることを要求
//     {
//         let mut new_graph = vec![vec![]; graph.len()];

//         for v in 0..graph.len() {
//             for i in 0..graph[v].len() {
//                 let nv = graph[v][i].0;
//                 let weight = graph[v][i].1.into(); // `Into`トレイトのメソッドを使用;
//                 new_graph[v].push(Edge::new(nv, weight));
//             }
//         }
//         return new_graph
//     }

//     // BinaryHeapの根を最大値ではなく最小値にするために構造体を書き換える
//     #[derive(Copy, Clone, Eq, PartialEq)]
//     struct State {
//         cost: usize,
//         position: usize,
//     }

//     // The priority queue depends on `Ord`.
//     // Explicitly implement the trait so the queue becomes a min-heap
//     // instead of a max-heap.
//     // impl トレイト名 for 構造体名
//     impl Ord for State {
//         fn cmp(&self, other: &Self) -> Ordering {
//             other.cost.cmp(&self.cost)
//                 .then_with(|| self.position.cmp(&other.position))
//         }
//     }

//     // `PartialOrd` needs to be implemented as well.
//     // impl トレイト名 for 構造体名
//     impl PartialOrd for State {
//         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//             Some(self.cmp(other))
//         }
//     }
// }