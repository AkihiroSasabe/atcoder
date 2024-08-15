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
    // 2024-08-15 10:41-11:14 (33min)
    // 【UnionFIndTree】
    // 順列Pが与えられている。M個の操作を何回使ってもいいので、P[I] = I と出来る最大数を答える問題。
    // M個の操作は、P[Xi]とP[Yi] を入れ替えるもの。
    // 操作で繋がれた数字同士は自由に行き来できることに気がつけば、あとはUnionFindで連結成分を管理して、
    // その連結成分に、所望のindexがあるかどうかをSetなどで答えられれば良い。

    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
    }
    p.iter_mut().for_each(|x| *x -= 1);
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..m {
        input! {
            xi: usize,
            yi: usize,
        }
        x.push(xi);
        y.push(yi);
    }
    x.iter_mut().for_each(|x| *x -= 1);
    y.iter_mut().for_each(|x| *x -= 1);

    // ノードで繋がれた点だけは行き来できる。

    // 連結成分内で合っているものを割り当てるだけ。
    
    // マッチング問題を解く
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        let xi = x[i];
        let yi = y[i];
        uft.unite(p[xi], p[yi]);
    }

    let mut hash = BTreeMap::new();
    let mut groups = vec![vec![]; n];
    for i in 0..n {
        let r = uft.root(p[i]);
        hash.entry(r).or_insert(BTreeSet::new()).insert(i);
        groups[r].push(p[i]);
    }

    // println!("hash = {:?}", hash);
    // println!("groups = {:?}", groups);

    let mut ans = 0;
    for r in 0..n {
        for i in 0..groups[r].len() {
            let v = groups[r][i];
            if hash.get(&r).unwrap().contains(&v) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
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
