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
    // 2024-09-24 20:06-20:30 (24min)
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n-1 {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
    }

    let subtree_sizes = get_subtree_sizes(0, &graph);
    let lca = LowestCommonAncestor::new(&graph, 0);

    // vs: 先行 と vt: 後攻 の位置をシミュレーションする。
    // vs と vt の最善手は、vs-vtパス上を移動してお互いに近づくこと。 (<- シミュレーションはここまででok)
    // これ以上、近づけなくなったら、残りの点群をじっくり取れば良い。
    let mut vs = 0;
    let mut vt = n-1;
    loop {
        let mut is_finished = false;
        for &nv in graph[vs].iter() {
            if nv == vt {
                is_finished = true;
                break
            }
            if lca.is_on_path(nv, vs, vt) {
                vs = nv;
                break
            }
        }

        if is_finished {break}
        let nv = lca.parent[0][vt];
        if nv == vs {break}
        vt = nv;
    }

    // 後攻の到達可能な個数
    let size = subtree_sizes[vt];
    if n - size > size {
        // 先行の勝ち
        println!("Fennec");
    }
    else {
        // 後行の勝ち
        println!("Snuke");
    }

}

/// 各頂点の「部分木の大きさのリストを返す関数
/// root_v := 木の根となる頂点
fn get_subtree_sizes(root_v: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // subtree_sizes[v] := 頂点vを根とする、部分木の大きさ(頂点数)
    let n = graph.len();
    let mut subtree_sizes = vec![0; n];
    let mut seen = vec![false; n];
    dfs(root_v, &graph, &mut seen, &mut subtree_sizes);
    return subtree_sizes
}
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, subtree_sizes: &mut Vec<usize>) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let nv = graph[v][i];
        if seen[nv] {continue}
        dfs(nv, graph, seen, subtree_sizes);
        subtree_sizes[v] += subtree_sizes[nv];
    }
    subtree_sizes[v] += 1;
}

// 参考実装: https://algo-logic.info/lca/
// ■LCA: 木の頂点uと頂点vの最近共通祖先xを求めるデータ構造 (ダブリングを使わなくても2分探索できると思うけど、一応ダブリングで実装)
// ■前処理
// 1. DFSで全ての頂点について、根からの距離と親を求める
// 2. 2^k先の祖先を求める
// ■クエリ(uとvの最近共通祖先xを求める)
// 1. uとvの深い方の頂点を近い方の頂点と同じ深さになるように変更して(u1, v1)とする。
// 2. 2分探索を用いて、u1,v1を最近共通祖先xの一つ手前まで段階的に移動させる
// (u1,v1が同じにならないギリギリまで移動させる)
// 3. 最終的なu1, v1の一つ先がLCA
// ■応用1: 2点u,vの距離もdist(u) - dist(v) - 2*dist(x)でO(logN)で求まる。
// ■応用2: 点aがxとyの間にあるかもdist(u,a)+dist(a,v)==dist(u,v)でO(logN)で判定可能
#[derive(Debug)]
struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>, // parent[k][u]:= u の 2^k個上の親
    distance: Vec<usize>     // rootからの距離
}
impl LowestCommonAncestor {
    pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> Self {
        let g_size = graph.len();
        // 最終的に2^k < g_sizeとなるような最小のk (=: k_over)を求める
        let mut k_over = 1; 
        while (1 << k_over) < g_size {
            k_over += 1;
        }
        let mut parent = vec![vec![g_size; g_size]; k_over]; // 親として存在しない頂点g_sizeをつけておく
        let mut distance = vec![0; g_size];
        
        // 一般的なダブリングの前処理
        // 1. それぞれの要素について、1個先の要素が何か記録 (今回はDFS)
        // 2. 前の結果を利用して、それぞれの要素について 2 個先の要素が何か記録
        // 3. 前の結果を利用して、それぞれの要素について 4 個先の要素が何か記録
        // 4. 前の結果を利用して、それぞれの要素について 8 個先の要素が何か記録
        // ...
        // k. 前の結果を利用して、それぞれの要素について 2^k 個先の要素が何か記録

        // 1. DFSで全ての頂点について、根からの距離と親を求める
        LowestCommonAncestor::get_dist_from_root_by_dfs(graph, root, g_size, 0, &mut distance, &mut parent);
        // 2. 2^k個上の祖先を求める(前処理の計算量O(NlogK))
        for k in 0..k_over-1 {
            for v in 0..g_size {
                // vの2^k個上の祖先が存在しない場合
                if parent[k][v] == g_size {
                    parent[k+1][v] = g_size;
                }
                else {
                    parent[k+1][v] = parent[k][parent[k][v]];
                }
            }
        }
        LowestCommonAncestor {parent, distance}
    }

    // DFSで全ての頂点vについて、根からの距離と親(1つ上の頂点)を求める
    fn get_dist_from_root_by_dfs(
        graph: &Vec<Vec<usize>>, // graph[v][i] := 頂点vとi番目に繋がっている頂点
        v: usize, // 頂点v
        p: usize, // 頂点vの親p
        d: usize, // 根と頂点vの距離d
        distance: &mut Vec<usize>,   // distance[v] := 頂点vと根からの距離
        parent: &mut Vec<Vec<usize>> // parent[k][v] := vの2^k個上の祖先
    )
    {
        distance[v] = d;
        parent[0][v] = p; // 頂点vの2^0=1個上の親をpとする
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            // 逆流しないようにする(子から親へ進まないようにする)
            if next_v != p {
                LowestCommonAncestor::get_dist_from_root_by_dfs(graph, next_v, v, d + 1, distance, parent);
            }
        }
    }

    // 頂点uと頂点vのLCA(最近共通祖先)を求める
    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        // uの方が深いようにする
        if self.distance[u] < self.distance[v] {
            swap(&mut u, &mut v);
        }
        // 頂点までの距離を同じにする
        let k_over = self.parent.len();
        let diff = self.distance[u] - self.distance[v];
        for k in 0..k_over {
            if (diff & (1 << k)) != 0 {
                u = self.parent[k][u];
            }
        }
        // LCA(最近共通祖先を求める)
        //         0
        //    1          2
        //  3   4     5     6
        // 7 8 9 10 11 12 13 14
        // 例えば上の木の7と10の最近共通祖先は1

        // 2分探索で求める
        if u == v {return u}
        for i in 0..k_over {
            let k = k_over - 1 - i;
            if self.parent[k][u] != self.parent[k][v] {
                // 2^k個上が、共通祖先じゃなかったら上がっていく
                // (最近共通祖先じゃなくても上がっていく)
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        // forループが終わると、uとvは最近共通祖先の1個下に居る。
        // (任意の数字は2の累乗の和で表現できるので、今回の"uから最近共通祖先までの距離-1"まで上がることも可能)
        // 返すべきは最近共通祖先なので、今のuの1=2^0個上の頂点を返す
        return self.parent[0][u];

        // 2分探索で求める
        // let mut ng: usize = 1 << (k_over-1); // uとvのng個上の頂点は共通祖先
        // let mut ok: usize = 0;        // uとvのok個上の頂点は共通祖先ではない
        // while (ng as isize - ok as isize).abs() < 1 {
        //     let mid = (ok + ng) / 2;
        //     let mut u_temp = u;
        //     let mut v_temp = v;
        //     for k in 0..k_over {
        //         if (mid & (1 << k)) != 0 {
        //             u_temp = self.parent[k][u_temp];
        //             v_temp = self.parent[k][v_temp];
        //         }
        //     }
        //     if u_temp != v_temp {
        //         ok = mid;
        //     }
        //     else {
        //         ng = mid;
        //     }
        // }
        // return ok + 1
    }

    // 頂点uと頂点vの距離を求める
    pub fn get_distance(&self, u: usize, v: usize) -> usize {
        let lca = self.query(u, v);
        let dist_from_u_to_lca = self.distance[u] - self.distance[lca];
        let dist_from_v_to_lca = self.distance[v] - self.distance[lca];
        return dist_from_u_to_lca + dist_from_v_to_lca
    }

    // 頂点xが頂点uと頂点vを結ぶパス上に存在するか?
    pub fn is_on_path(&self, x: usize, u:usize, v: usize) -> bool {

        return self.get_distance(u, x) + self.get_distance(x, v) == self.get_distance(u, v)
        
        // (下記でも良さそう.※要検証)
        // let lca_xu = self.query(x, u);
        // let lca_xv: usize = self.query(x, v);
        // return lca_xu == x || lca_xv == x

    }

}

