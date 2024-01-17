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
    // 2024-01-14 21:51-22:40 (49min)
    // 2024-01-15 19:08-19:24 (12min, 解説ac)
    // total 61min
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp_l = vec![0; n];
    let mut dp_r = vec![0; n];
    dp_l[0] = 1;
    dp_r[n-1] = 1;
    for i in 1..n {
        dp_l[i] = min(dp_l[i-1] + 1, a[i]);
    }
    for i in (0..n-1).rev() {
        dp_r[i] = min(dp_r[i+1] + 1, a[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, min(dp_l[i], dp_r[i]));
    }
    // println!("dp_l = {:?}", dp_l);
    // println!("dp_r = {:?}", dp_r);
    println!("{}", ans);


    // let mut uft = UnionFindTree::new(n);
    // let mut a_sort = vec![];
    // for i in 0..n {
    //     a_sort.push([a[i], i]);
    // }
    // // let mut a_sort = a.clone();
    // a_sort.sort();
    // a_sort.reverse();

    // let mut seen = vec![false; n];
    // let mut ans = 0;
    // for i in 0..n {
    //     let val = a_sort[i][0];
    //     let ind = a_sort[i][1];
    //     seen[ind] = true;

    //     if ind != 0 && seen[ind] && seen[ind-1] {
    //         uft.unite(ind, ind-1);
    //     }
    //     if ind != n -1 && seen[ind] && seen[ind+1] {
    //         uft.unite(ind, ind+1);
    //     }
    //     let r = uft.root(ind);
    //     let l = 2 * val - 1;
    //     let size = uft.size(r);
    //     println!("i ={i}, val = {val}----");
    //     println!("l = {:?}", l);
    //     println!("size = {:?}", size);
    //     let real_l = min(size, l);
    //     let real_k = (real_l + 1) / 2;
    //     println!("real_l = {:?}", real_l);
    //     println!("real_k = {:?}", real_k);
    //     ans = max(ans, real_k);
    // }
    // println!("{}", ans);


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
