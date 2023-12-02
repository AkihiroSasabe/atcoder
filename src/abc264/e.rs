use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, BTreeSet, HashSet};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    // 2023-12-02 17:12-18:01 (49min)
    input! {
        n: usize,
        m: usize,
        e: usize,
    }
    let mut edges = vec![];
    for i in 0..e {
        input! {
            u_i: usize,
            v_i: usize,
        }
        edges.push(vec![u_i-1, v_i-1]);
    }
    input!{
        q: usize
    }
    let mut x = vec![];
    let mut cutters = HashSet::new();
    for i in 0..q {
        input!{
            x_i: usize
        }
        x.push(x_i-1);
        cutters.insert(x_i-1);
    }
    // 全発電所を繋ぐ超新星を n + m として追加
    let mut uft = UnionFindTree::new(n + m + 1);
    for i in n..n+m {
        uft.unite(i, n+m);
    }

    for i in 0..e {
        if cutters.contains(&i) {
            continue
        }
        else {
            let u = edges[i][0];
            let v = edges[i][1];
            uft.unite(u, v);
        }
    }

    let mut ans = vec![];
    for i in (0..q).rev() {
        ans.push(uft.size(n+m) - m - 1);
        let u = edges[x[i]][0];
        let v = edges[x[i]][1];
        uft.unite(u, v);
    }

    ans.reverse();
    for i in 0..q {
        println!("{}", ans[i]);
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
