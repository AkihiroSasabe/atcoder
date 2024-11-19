#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
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
    // 2024-11-18 20:34-20:55 (問題文の意味の理解)
    // 
    input! {
        n: usize,
        k: usize,
        l: usize,
    }

    // 道路
    let mut p = vec![];
    let mut q = vec![];
    for i in 0..k {
        input!{
            pi: Usize1,
            qi: Usize1,
        }
        p.push(pi);
        q.push(qi);
    }

    // 鉄道
    let mut r = vec![];
    let mut s = vec![];
    for i in 0..l {
        input!{
            ri: Usize1,
            si: Usize1,
        }
        r.push(ri);
        s.push(si);
    }
    // solve(n, k, l, p, q, r, s);
    solve_brute_force(n, k, l, p, q, r, s);
}

fn solve(n: usize, k: usize, l: usize, p: Vec<usize>, q: Vec<usize>, r: Vec<usize>, s: Vec<usize>) {
    let mut uft = UnionFindTree::new(n);
    let mut uft2 = UnionFindTree::new(n);
    let mut uft3 = UnionFindTree::new(n);

    for i in 0..k {
        uft.unite(p[i], q[i]);
    }
    for i in 0..l {
        uft2.unite(r[i], s[i]);
    }


    for i in 0..k {
        if uft2.issame(p[i], q[i]) {
            uft3.unite(p[i], q[i]);
        }
    }
    for i in 0..l {
        if uft.issame(r[i], s[i]) {
            uft3.unite(r[i], s[i]);
        }
    }

    for i in 0..n {
        let ans = uft3.size(i);
        print!("{} ", ans);
    }
}

fn solve_brute_force(n: usize, k: usize, l: usize, p: Vec<usize>, q: Vec<usize>, r: Vec<usize>, s: Vec<usize>) {
    let mut uft = UnionFindTree::new(n);
    let mut uft2 = UnionFindTree::new(n);
    let mut uft3 = UnionFindTree::new(n);

    for i in 0..k {
        uft.unite(p[i], q[i]);
    }
    for i in 0..l {
        uft2.unite(r[i], s[i]);
    }
    for i in 0..n {
        for j in i+1..n {
            if uft.issame(i, j) && uft2.issame(i, j) {
                uft3.unite(i, j);
            }
        }
    }
    for i in 0..n {
        let ans = uft3.size(i);
        print!("{} ", ans);
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
