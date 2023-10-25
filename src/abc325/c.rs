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
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut holes = BTreeSet::new();
    let mut uft = UnionFindTree::new(h*w);
    // 左上、上、右上、 左、右、 左下、下、右下
    let mut dy: Vec<isize> = vec![-1,-1,-1,  0,0,   1,1,1];
    let mut dx: Vec<isize> = vec![-1,0,1,   -1,1,  -1,0,1];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                holes.insert(y * w + x);
                for i in 0..dx.len() {
                    let next_y = y as isize + dy[i];
                    let next_x = x as isize + dx[i];
                    if next_y < 0 || h as isize <= next_y || next_x < 0 || w as isize <= next_x {continue}
                    if s[next_y as usize][next_x as usize] == '#' {
                        let next_val = (next_y as usize * w) + (next_x as usize);
                        uft.unite(y * w + x, next_val);
                    }
                }
            }
        }
    }

    let mut neo_holes = BTreeSet::new();
    for v in holes.iter() {
        neo_holes.insert(uft.root(*v));
    }
    println!("{}", neo_holes.len());


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
