#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-06-23 19:38-21:11 (93min)
    input! {
        n: usize,
        m: usize,
    }
    let mut u = vec![];
    let mut v = vec![];
    // 入次数
    let mut g: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n+1];
    // let mut g: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for i in 0..m {
        input!{
            // ui: Usize1,
            // vi: Usize1,
            ui: usize,
            vi: usize,
        }
        u.push(ui);
        v.push(vi);
        g[ui].insert(vi);
        g[vi].insert(ui);
    }
    input!{
        q: usize,
        x: [Usize1; q]
    }

    let mut uft = UnionFindTree::new(g, m);
    // println!("uft.g = {:?}", uft.g);

    for i in 0..q {
        let xi = x[i];
        let ui = u[xi];
        let vi: usize = v[xi];

        uft.unite(ui, vi);
        let ans = uft.num_edges;
        // println!("uft.g = {:?}", uft.g);
        println!("{}", ans);
    }


}


// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
#[derive(Clone)]
struct UnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>,       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
    g: Vec<BTreeSet<usize>>,
    num_edges: usize
}

impl UnionFindTree {
    // 初期化
    fn new(g: Vec<BTreeSet<usize>>, num_edges: usize) -> Self {

        let n = g.len();
        // 各頂点が属するグループの根を格納
        let parents = (0..n).collect();
        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes = vec![1; n];
        return UnionFindTree {parents, sizes, g, num_edges}
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

        if self.g[v0].len() <= self.g[v1].len() {
            child = v0;
            parent = v1;
        }
        else {
            child = v1;
            parent = v0;
        }

        // v0-v1を消す
        if self.g[child].contains(&parent) {
            self.g[child].remove(&parent);
            self.g[parent].remove(&child);
            self.num_edges -= 1;
        }
        let mut moves = vec![];
        for &nv in self.g[child].iter() {
            moves.push(nv);
        }

        // childに繋がれている辺を parentに繋ぎ替える。
        for nv in moves {
            if self.g[parent].contains(&nv) && self.g[child].contains(&nv) {
                self.num_edges -= 1;                
            }
            self.g[child].remove(&nv);
            self.g[nv].remove(&child);
            self.g[parent].insert(nv);
            self.g[nv].insert(parent);

        }
        self.parents[child] = parent;
        return true
    }
    // fn print_for_debug(&mut self) {
    //     let n = self.sizes.len();
    //     let mut lists = vec![vec![]; n];
    //     for v in 0..n {
    //         let r = self.root(v);
    //         lists[r].push(v);
    //     }
    //     println!("uft = {:?}", lists);
    //     // print!("uft: ");
    //     // for v in 0..n {
    //     //     print!("({}: {:?}), ", v, lists[v]);
    //     // }
    //     // println!("");
    // }
}
