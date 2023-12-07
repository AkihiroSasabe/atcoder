#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::btree_set::Union;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;

fn main() {
    // 2023-12-07 15:53-16:50 (57min)
    input! {
        n: usize,
        k: usize,
        xy: [(isize, isize); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut uft = UnionFindTree::new(n * (n-1) /2);
    let mut ei = 0;
    let mut edge = vec![];
    for i in 0..n {
        for j in i+1..n {
            let xi = xy[i].0;
            let yi = xy[i].1;
            let xj = xy[j].0;
            let yj = xy[j].1;
            // 
            edge.push((
                (xi, yi),
                (xi - xj, yi - yj)
            ));
            ei += 1;
        }
    }

    let mut hash = HashSet::new();
    for i in 0..edge.len() {
        let mut num: usize = 0;
        let mut set = vec![];
        for v in 0..n {
            let xv = xy[v].0;
            let yv = xy[v].1;
            let xi = edge[i].0.0;
            let yi = edge[i].0.1;
            let dxi = edge[i].1.0;
            let dyi = edge[i].1.1;
            // 外積を求める
            let cross = get_cross(xv - xi, yv - yi, dxi, dyi);
            // 外積が0なら、その頂点は直線上にあるといえる。
            if cross == 0 {
                num += 1;
                set.push(v);
            }
        }
        if num >= k {
            set.sort();
            hash.insert(set);
        }
    }
    
    println!("{}", hash.len());

}

fn get_cross(x0: isize, y0: isize, x1: isize, y1: isize) -> isize {
    return x0 * y1 - y0 * x1
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
