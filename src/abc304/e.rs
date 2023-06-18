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
use std::collections::HashSet;
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
    // 多重辺を弾くための連想配列
    let mut hash = HashMap::new();
    // UFTで連結成分をまとめる
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        // 自己ループを除去
        if u_i == v_i {continue}
        // 多重辺を除去
        if hash.contains_key(&vec![u_i, v_i]) {
            continue
        }
        else {
            hash.insert(vec![u_i, v_i], 1);
        }

        graph[u_i-1].push(v_i-1);
        graph[v_i-1].push(u_i-1);
        uft.unite(u_i-1, v_i-1);
    }
    input! {
        k: usize
    }
    let mut bans = vec![];
    for i in 0..k {
        input! {
            x_i: usize,
            y_i: usize,
        }
        bans.push(vec![x_i-1, y_i-1])
    }

    let mut pq = vec![];
    input! {
        q: usize
    }
    for i in 0..q {
        input! {
            p_i: usize,
            q_i: usize,
        }
        pq.push(vec![p_i-1, q_i-1]);
    }

    // 繋いではいけない辺を連結成分単位で格納
    let mut ng_hash = HashMap::new();

    for i in 0..k {
        let x = bans[i][0];
        let y = bans[i][1];
        let root_x = uft.root(x);
        let root_y = uft.root(y);
        ng_hash.insert(vec![root_x, root_y], 0);
        ng_hash.insert(vec![root_y, root_x], 0);
    }

    // 各クエリの頂点同士の辺を、連結成分同士の辺に置き換えてNGか判定
    for i in 0..q {
        let p = pq[i][0];
        let q = pq[i][1];
        let root_p = uft.root(p);
        let root_q = uft.root(q);
        if ng_hash.contains_key(&vec![root_p, root_q]) {
            println!("No");
            continue
        }
        else {
            println!("Yes");
        }
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