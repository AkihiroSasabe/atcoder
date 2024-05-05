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
        k: usize,
        p: [usize; n]
    }
    // ソート　あんど　尺取
    let mut pp = vec![];
    for i in 0..n {
        pp.push([p[i], i+1]);
    }
    pp.sort();
    // println!("pp = {:?}", pp);


    let mut uft = UnionFindTree::new(n);
    for i in 0..n-1 {
        if pp[i][0] + 1 == pp[i+1][0] {
            uft.unite(i, i+1);
        }
    }
    let mut ans = 1 << 60;
    let mut checked = HashSet::new();
    for i in 0..n {
        if checked.contains(&i) {continue}
        checked.insert(i);
        if uft.size(i) >= k {
            let mut left = i;
            let mut set = BTreeSet::new();
            let mut right = 0;
            for v in left..left+k {
                set.insert(pp[v][1]);
                checked.insert(v);
                right = v;
            }
            let i0 = set.iter().next().unwrap();
            let ik = set.iter().rev().next().unwrap();
            ans = min(ans, ik - i0);
            for _ in 0..(uft.size(i) - k) {
                right += 1;
                checked.insert(right);

                set.insert(pp[right][1]);
                set.remove(&pp[left][1]);
                left += 1;

                let i0 = set.iter().next().unwrap();
                let ik = set.iter().rev().next().unwrap();
                ans = min(ans, ik - i0);
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
