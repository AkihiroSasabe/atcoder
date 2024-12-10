#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
    // 2024-12-07 22:17-22:40 (23min)
    // 2024-12-08 11:24-12:42 (78min)
    // 2024-12-08 15:05-16:04 (59min)
    // Total: 161 min
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut edges = vec![];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
            wi: usize,
        }
        edges.push((wi, ui, vi));
    }

    input! {
        a: [Usize1; k],
        b: [Usize1; k],
    }
    let mut uft = UnionFindTree::new(n);
    let mut ans = 0;

    // ass[v] := 頂点v　が持つ、aの個数
    // bss[v] := 頂点v　が持つ、bの個数
    let mut ass = vec![vec![]; n];
    let mut bss = vec![vec![]; n];
    for i in 0..k {
        ass[a[i]].push(i);
        bss[b[i]].push(i);
    }
    // println!("ass = {:?}", ass);
    // println!("bss = {:?}", bss);

    edges.sort();
    // 重みの小さなエッジからやっていく
    for (wi, ui, vi) in edges {
        // println!("******* edge = ui={}, vi={}, wi={:?}", ui,vi,wi);

        let ui = uft.root(ui);
        let vi = uft.root(vi);

        let child: usize;
        let parent: usize;
        if uft.size(ui) <= uft.size(vi) {
            child = ui;
            parent = vi;
        }
        else {
            child = vi;
            parent = ui;
        }

        // println!("child = {}, parent = {:?}", child, parent);

        if child != parent {
            // 移植する　子 -> 親
            let src = ass[child].clone();
            for ind in src {
                ass[parent].push(ind);
            }
            ass[child] = vec![];

            // 移植する　子 -> 親
            let src = bss[child].clone();
            for ind in src {
                bss[parent].push(ind);
            }
            bss[child] = vec![];
        }

        // println!("[insert] ass = {:?}", ass);
        // println!("[insert] bss = {:?}", bss);

        // 結合していく。
        uft.unite(ui, vi);

        // 同じ島にAとBのペアはおるんか?
        while ass[parent].len() > 0 && bss[parent].len() > 0 {
            ans += wi;
            let ai = ass[parent].pop().unwrap();
            let bi = bss[parent].pop().unwrap();
            // println!("ai = {}, bi = {:?}, wi = {}", ai, bi, wi);
        }
        // println!("[delete] ass = {:?}", ass);
        // println!("[delete] bss = {:?}", bss);
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