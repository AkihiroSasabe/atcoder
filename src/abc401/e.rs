#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-04-13 11:31-13:14
    
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
        }
        graph[ui-1].push(vi-1);
        graph[vi-1].push(ui-1);
    }

    // k=0のとき
    let mut ans = graph[0].len();
    let mut removed = BTreeSet::new();
    for &nv in graph[0].iter() {
        removed.insert(nv);
    }
    println!("{}", ans);

    let mut uft = UnionFindTree::new(n);
    // dpかな
    for k in 1..n {
        
        let mut pre = ans;
        let mut is_ok = false;

        // k から湧き出るやつを消す。
        for i in 0..graph[k].len() {
            let nv =  graph[k][i];
            if nv < k {
                uft.unite(nv, k);
            }
            else {
                removed.insert(nv);
            }
        }
        removed.remove(&k);
        // println!("k = {k}, removed = {:?}", removed);

        // uft.print_for_debug();
        if uft.issame(0, k) && uft.size(k) == k + 1 {
            println!("{}", removed.len());
        }
        else {
            println!("-1");
        }
    }

    
}



// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
#[derive(Clone)]
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
    fn print_for_debug(&mut self) {
        let n = self.sizes.len();
        let mut lists = vec![vec![]; n];
        for v in 0..n {
            let r = self.root(v);
            lists[r].push(v);
        }
        println!("uft = {:?}", lists);
        // print!("uft: ");
        // for v in 0..n {
        //     print!("({}: {:?}), ", v, lists[v]);
        // }
        // println!("");
    }
}
