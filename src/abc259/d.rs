use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        n: usize,
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut r = vec![];

    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
            r_i: isize,
        }
        x.push(x_i);
        y.push(y_i);
        r.push(r_i);
    }

    let mut uft = UnionFindTree::new(n);
    for i in 0..n {
        for j in i+1..n {
            let dist = ((x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]));
            if dist <= (r[i] + r[j]) * (r[i] + r[j]) && (r[i] - r[j]).abs() * (r[i] - r[j]).abs() <= dist {
                uft.unite(i, j);
            }
        }
    }
    let mut s_index: usize = 0;
    let mut t_index: usize = 0;

    for i in 0..n {
        let dist_s = ((x[i] - sx) * (x[i] - sx) + (y[i] - sy) * (y[i] - sy));
        let dist_t = ((x[i] - tx) * (x[i] - tx) + (y[i] - ty) * (y[i] - ty));
        if dist_s == r[i] * r[i]{
            s_index = i
        }
        if dist_t == r[i] * r[i] {
            t_index = i
        }
    }

    // println!("s_index {}", s_index);
    // println!("t_index {}", t_index);

    let flag = uft.issame(s_index, t_index);

    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }


}



// Union-Find
struct UnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>       // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
}

// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
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
