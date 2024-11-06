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
    // 2024-07-22 20:50-21:00 (10min)
    // 2024-07-23 19:25-20:01 (36min, 初提出WA)
    // 2024-07-23 20:01-21:00 (59min, デバッグ)
    // 2024-07-24 12:20-12:33 (13min, AC)
    // Total: 118min
    input! {
        h: usize,
        w: usize,
        year: usize,
        a: [[usize; w]; h]
    }
    let max_year = 100_000;
    let mut set = BTreeSet::new();
    for i in 0..h {
        for j in 0..w {
            set.insert((a[i][j], i, j));
        }
    }

    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    let mut uft = UnionFindTree::new(h*w);
    let mut is_umi_list = vec![false; h*w];
    let mut ans = h * w;

    let mut years = vec![h*w; max_year + 1];

    for (v, y, x) in set {
        // println!("v = {:?}, ans = {ans}", v);
        let r = uft.root(y * w + x);
        let is_pre_umi = is_umi_list[r];

        if !is_pre_umi {
            // 自分自身が、これから海になりうるか?
            for i in 0..dir_y.len() {
                let ny = dir_y[i] + y as isize;
                let nx = dir_x[i] + x as isize;

                // 周囲が大海
                if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {
                    is_umi_list[r] = true;
                    ans -= uft.size(r); // ここを -= 1 にしていたせいで、WAだった。
                    break
                }

                let ny = ny as usize;
                let nx = nx as usize;
                let nr = uft.root(ny * w + nx);

                // 周囲が小海か?
                if is_umi_list[nr] {
                    is_umi_list[r] = true;
                    ans -= uft.size(r);
                    break
                }
            }
        }

        // 周囲と連結できるか?
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;

            // 周囲が大海
            if ny < 0 || h as isize <= ny || nx < 0 || w as isize <= nx {
                continue
            }

            let ny = ny as usize;
            let nx = nx as usize;
            let nr = uft.root(ny * w + nx);

            // 連結可能か?
            if v >= a[ny][nx] {
                // 自分自身が海で、隣人が海じゃないとき、海にしずめる
                if is_umi_list[r] && !is_umi_list[nr] {
                    // 
                    ans -= uft.size(nr);
                    is_umi_list[nr] = true;
                }

                // 連結
                uft.unite(r, nr);
            }
        }
        years[v] = ans;
        // println!("ans = {ans}");
    }
    for i in 1..year+1 {
        years[i] = min(years[i], years[i-1]);
        println!("{}", years[i]);
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
