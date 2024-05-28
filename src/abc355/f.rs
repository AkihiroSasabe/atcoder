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
    // 2024-05-26 10:58-11:45 (47min, 徒労)
    // 2024-05-27 20:44-21:54 (70min, 解説AC.)
    input! {
        n: usize,
        q: usize
    }
    let mut abc = vec![];
    for i in 0..n-1 {
        input!{
            ai: usize,
            bi: usize,
            ci: usize,
        }
        abc.push((ai, bi, ci));
    }
    let mut uvw = vec![];
    for i in 0..q {
        input!{
            ui: usize,
            vi: usize,
            wi: usize,
        }
        uvw.push((ui, vi, wi));
    }

    // 重みは、 ci か...
    // 重みは、 
    // 重み、ソートして....
    // wより重い重みが何本あるか?
    // 連結かどうかの判定。

    // Cons(k) := 重みが k 以下の辺からなるグラフの連結成分の個数
    // 例: 入力1の初期状態なら...
    // Cons(0) = 4
    // Cons(1) = 4
    // Cons(2) = 4
    // Cons(3) = 3 <-
    // Cons(4) = 2 <-
    // Cons(5) = 2
    // Cons(6) = 1 <-
    // Cons(7) = 1
    // Cons(8) = 1
    // Cons(9) = 1
    // Cons(10) = 1

    // 最小全域木の重みS
    // S = Σ [k=1,2,...10] (Cons(k-1) - Cons(k)) * k // 重みを変えて連結成分数が減ったとき、それが最小全域木に寄与する重みになる。
    //   = Σ [k=1,2,...10] k * Cons(k-1) - k * Cons(k)
    //   = Σ [k=1,2,...10] (k-1) * Cons(k-1) - kCons(k) + Cons(k-1)
    //   = 0 * Cons(0) - 1 * Cons(1) + Cons(0)
    //   + 1 * Cons(1) - 2 * Cons(2) + Cons(1)
    //      ...
    //   + 9 * Cons(9) - 10 * Cons(10) + Cons(9)
    //   = 0 * Cons(0) - 10 * Cons(10) + Σ [k=0,1,2,...9] Cons(k)
    //   = n - 10 + Σ [k=1,2,...9] Cons(k) (∵ Cons(10) = 1, Cons(0) = n)

    // uft を10本生やせばいいのか?

    let mut ufts = vec![UnionFindTree::new(n); 11];
    let mut num_comps = vec![n; 11];
    for i in 0..n-1 {
        // println!("----- i = {i} ====");
        let ai = abc[i].0;
        let bi = abc[i].1;
        let ci = abc[i].2;
        for weight in 1..11 {
            if ci > weight {continue}
            if !ufts[weight].issame(ai-1, bi-1) {
                num_comps[weight] -= 1;
                ufts[weight].unite(ai-1, bi-1);
                // println!("weight = {weight}, ai = {}, bi = {}, num_comps[{weight}] = {}", ai, bi, num_comps[weight]);
            }
        }
        // println!("num_comps = {:?}", num_comps);
    }
    
    // println!("----- init done =====");
    // println!("num_comps = {:?}", num_comps);

    for i in 0..q {
        // println!("---- query = {i}");
        let (ui, vi, wi) = uvw[i];
        for weight in wi..11 {
            if !ufts[weight].issame(ui-1, vi-1) {
                num_comps[weight] -= 1;
                ufts[weight].unite(ui-1, vi-1);
            }
        }
        // ans = n -10 + Σ [k=1,2,...9] Cons(k) 
        let mut ans = n;
        for i in 1..10 {
            ans += num_comps[i];
        }
        ans -= 10;
        // println!("num_comps = {:?}", num_comps);
        println!("{}", ans);
    }
}

#[derive(Clone)]
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
