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
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        input!{ 
            a_i: isize,
            b_i: isize,
            x_i: isize,
            y_i: isize,
        }
        graph[(a_i-1) as usize].push(vec![b_i-1, x_i, y_i]);
        graph[(b_i-1) as usize].push(vec![a_i-1, -x_i, -y_i]);
        uft.unite((a_i-1) as usize, (b_i-1) as usize);
    }

    let mut x = vec![0; n];
    let mut y = vec![0; n];
    let mut seen = vec![false; n];
    dfs(0, &graph, &mut seen, &mut x, &mut y);
    for i in 0..n {
        if uft.issame(0, i) {
            println!("{} {}", x[i], y[i]);
        }
        else {
            println!("undecidable");
        }
    }


}

fn dfs(v: usize, graph: &Vec<Vec<Vec<isize>>>, seen: &mut Vec<bool>, x: &mut Vec<isize>, y: &mut Vec<isize>) {
    seen[v] = true;
    for i in 0..graph[v].len() {
        let next_v = graph[v][i][0] as usize;
        let dx = graph[v][i][1];
        let dy = graph[v][i][2];
        x[next_v] = x[v] + dx;
        y[next_v] = y[v] + dy;
        if seen[next_v] {continue}
        dfs(next_v, graph, seen, x, y);
    }
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
