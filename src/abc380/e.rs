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
    // 2024-11-16 22:07-22:40 (33min)
    // 2024-11-17 10:46-11:22 (36min)
    // Total: 69min
    input! {
        n: usize,
        q: usize,
    }
    let mut uft = UnionFindTree::new(n);
    let mut nums = vec![1; n]; // nums[c] := c色の数
    let mut colors = vec![]; // colors[x] := xの色
    for i in 0..n {
        colors.push(i);
    }
    
    for i in 0..q {
        input! {
            op: usize
        }
        match op {
            1 => {
                input! {
                    xi: Usize1,
                    ci: Usize1,
                }
                // 塗る
                let ri = uft.root(xi);
                // もとの色を減らす
                nums[colors[ri]] -= uft.size(ri);         
                // 新しい色を増やす
                nums[ci] += uft.size(ri);      
                // 色を変える
                colors[uft.root(xi)] = ci;

                // 両隣がもともと同じ色で、同じグループだった場合、グループの両端で、結合判断すべき
                // 左側のチェック
                let &min_v = uft.get_set(xi).iter().next().unwrap();
                if min_v != 0 {
                    let nv = min_v - 1;
                    // マージ可能か？
                    let rn = uft.root(nv);
                    if colors[rn] == ci {
                        uft.unite(xi, nv);
                    }
                }
                // 右側のチェック
                let &max_v = uft.get_set(xi).iter().rev().next().unwrap();
                if max_v != n - 1 {
                    let nv = max_v + 1;
                    // マージ可能か？
                    let rn = uft.root(nv);
                    if colors[rn] == ci {
                        uft.unite(xi, nv);
                    }
                }
            },
            2 => {
                input! {
                    ci: Usize1
                }
                // 数える
                println!("{}", nums[ci]);
            },
            _ => {}
        }
    }
}



// UnionFindTreeWithSets: UFTで、頂点の集合も取得できるようにした。トータルの結合計算量が、O(Nlog(N))で、ちょっと遅くなっている。
// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
struct UnionFindTreeWithSets {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>,       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
    sets: Vec<BTreeSet<usize>>,    // 各頂点の属するグループ(根付き木)の集合
}

impl UnionFindTreeWithSets {
    // 初期化
    fn new(n: usize) -> Self {
        // 各頂点が属するグループの根を格納
        let parents = (0..n).collect();
        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes = vec![1; n];

        let mut sets = vec![BTreeSet::new(); n];
        for i in 0..n {
            sets[i].insert(i);
        }
        return UnionFindTree {parents, sizes, sets}
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

        // `std::mem::take`で`self.sets[child]`の所有権を一時的に奪う
        let child_set = std::mem::take(&mut self.sets[child]);
        for ch in child_set {
            self.sets[parent].insert(ch);
        }

        return true
    }

    // 頂点vと同じグループの集合を取得する
    fn get_set(&mut self, v: usize) -> &BTreeSet<usize> {
        let r = self.root(v);
        return &self.sets[r]
    }
}
