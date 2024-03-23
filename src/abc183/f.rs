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
    // 2024-03-20 16:50-16:56 (6min)
    // 2024-03-21 20:48-21:32 (44min)
    // Total 50 min
    input! {
        n: usize,
        q: usize,
        mut c: [usize; n],
    }
    for i in 0..n {
        c[i] -= 1;
    }
    let mut k = vec![];
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input!{
            ki: usize,
            ai: usize,
            bi: usize,
        }
        k.push(ki);
        a.push(ai-1);
        b.push(bi-1);
    }

    let mut uft = UnionFindTree::new(n);
    let mut class = vec![HashMap::new(); n];
    for i in 0..n {
        class[i].insert(c[i], 1 as usize);
    }

    for i in 0..q {
        let ai = a[i];
        let bi = b[i];
        if k[i] == 1 {
            let ra = uft.root(ai);
            let rb = uft.root(bi);
            if ra == rb {continue}
            if uft.size(ai) <= uft.size(bi) {
                // bi にaiの全てをぶち込む
                for (cc, num) in class[ra].clone() {
                    *class[rb].entry(cc).or_insert(0) += num;
                }
            }
            else {
                // ai に bi の全てをぶち込む
                for (cc, num) in class[rb].clone() {
                    *class[ra].entry(cc).or_insert(0) += num;
                }
            }
            // println!("class = {:?}", class);
            // println!("uft = {:?}", uft);
            // 結合
            uft.unite(ai, bi);
        }
        else {
            // 生徒aiの集団で、クラスbiに属しているものの
            let ra = uft.root(ai);
            if let Some(ans) = class[ra].get(&bi) {
                println!("{}", ans);
            }
            else {
                println!("0");
            }
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
