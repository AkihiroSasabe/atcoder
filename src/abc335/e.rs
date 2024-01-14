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
    // 2024-01-07 16:51-
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut uft = UnionFindTree::new(n);
    let mut uv = vec![];
    for i in 0..m {
        input! {
            ui: usize,
            vi: usize,
        }
        let u = ui - 1;
        let v = vi - 1;
        if a[u] == a[v] {
            uft.unite(u, v);
        }
        uv.push([u,v]);
    }
    let mut graph  =vec![vec![]; n];
    let mut val_edges = vec![vec![]; 200_001];
    for i in 0..m {
        let u = uft.root(uv[i][0]);
        let v = uft.root(uv[i][1]);
        if a[u] < a[v] {
            graph[u].push(v);
            val_edges[a[u]].push(vec![u, v]);
        }
        else if a[u] > a[v] {
            graph[v].push(u);
            val_edges[a[v]].push(vec![v, u]);
        }
    }

    // あとはdpするだけ (BFSならTLE)
    // let dp: Vec<usize> = bfs(uft.root(0), &graph);
    let mut dp = vec![0; n];
    dp[uft.root(0)] = 1 as usize;
    for edges in val_edges {
        for edge in edges.iter() {
            let u = edge[0];
            let v = edge[1];
            if dp[u] > 0 {
                dp[v] = max(dp[v], dp[u]+1);
            }
        }
    }

    let ans = dp[uft.root(n-1)];
    println!("{}", ans);

}



fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init: usize = 0; // 10^18
    let mut queue = VecDeque::new();
    let mut dp = vec![init; graph.len()];
    dp[v0] = 1;
    queue.push_back(vec![v0, dp[v0]]);
    while queue.len() > 0 {
        let vec = queue.pop_front().unwrap();
        let v = vec[0];
        let val = vec[1];
        if dp[v] > val {continue}
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if dp[nv] >= dp[v] + 1 {continue}
            dp[nv] = dp[v] + 1;
            queue.push_back(vec![nv, dp[nv]]);
        }
    }
    return dp
}

// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
struct UnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
}

impl UnionFindTree {
    // 初期化
    fn new(n: usize) -> Self {
        // 各頂点が属するグループの根を格納
        let parents = (0..n).collect();
        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes = vec![1; n];
        return UnionFindTree {parents, sizes}
    }

    // 根を求める。経路圧縮により計算量を削減
    fn root(&mut self, v: usize) -> usize {
        if self.parents[v] == v {return v}
        else {
            // 経路圧縮 (親を根に張り替える。)
            self.parents[v] = self.root(self.parents[v]);
            return self.parents[v] as usize
        }
    }

    // 同じグループに属するか
    fn issame(&mut self, v0: usize, v1: usize) -> bool {
        return self.root(v0) == self.root(v1)
    }

    // 頂点vが属する根付き木のサイズを取得
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        return self.sizes[root]
    }

    // v0を含むグループと、v1を含むグループとを併合する。Union by sizeで計算量削減。
    fn unite(&mut self, mut v0: usize, mut v1: usize) -> bool {
        // 既に同じグループであれば何もしない
        v0 = self.root(v0);
        v1 = self.root(v1);
        if v0 == v1 {
            return false
        } 
        let child: usize;
        let parent: usize;
        // Union by sizeにより、サイズが小さいグループを、サイズが大きいグループに併合する
        if self.size(v0) <= self.size(v1) {
            child = v0;
            parent = v1;
        }
        else {
            child = v1;
            parent = v0;
        }
        self.sizes[parent] += self.size(child);
        self.parents[child] = parent;
        return true
    }
}

