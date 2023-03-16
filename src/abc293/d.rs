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
    let mut uft = UnionFindTree::new(2*n);
    let mut cin = vec![0; n+1];
    for i in 0..m {
        input! {
            mut a_i: usize,
            b_i: char,
            mut c_i: usize,
            d_i: char,
        }
        uft.unite(a_i, c_i);
        cin[a_i] += 1;
        cin[c_i] += 1;
    }
    let mut hash = HashMap::new();
    for i in 1..n+1 {
        let root = uft.root(i);
        if hash.contains_key(&root) {
            if cin[i] != 2 {
                *hash.get_mut(&root).unwrap() = false;
            }
            continue
        }
        else {
            hash.insert(root, cin[i] == 2);
        }
    }
    let mut x = 0;
    for (k,v) in &hash {
        if *v {
            x += 1;
        }
    }
    // println!("{:?}", hash);
    let y = hash.len() - x;
    println!("{} {}", x, y);


    // let mut uft = UnionFindTree::new(2*n);
    // for i in 0..n {
    //     // B-R
    //     uft.unite(i, n+i);
    // }

    // let mut cycle = vec![false; 2*n];
    
    // for i in 0..m {
    //     input! {
    //         mut a_i: usize,
    //         b_i: char,
    //         mut c_i: usize,
    //         d_i: char,
    //     }
    //     a_i -= 1;
    //     c_i -= 1;
    //     let a_i_B = a_i;
    //     let a_i_R = a_i+n;
    //     let c_i_B = c_i;
    //     let c_i_R = c_i+n;

    //     if b_i == 'R' {
    //         a_i += n;
    //     }
    //     if d_i == 'R' {
    //         c_i += n;
    //     }
    //     if uft.issame(a_i, c_i) {
    //         cycle[a_i_B]= true;
    //         cycle[a_i_R] = true;
    //         cycle[c_i_B] = true;
    //         cycle[c_i_R] = true;
    //     }
    //     uft.unite(a_i, c_i);
    // }
    // let mut x = 0;
    // let mut y = 0;
    // let mut hash = HashMap::new();

    // println!("cycle {:?}", cycle);
    // // サイクルである一繋ぎの個数xを数える
    // // y: 一繋ぎの個数
    // for i in 0..2*n {
    //     if hash.contains_key(&uft.root(i)) {continue}
    //     y += 1;
    //     hash.insert(uft.root(i), 0);
    //     if cycle[i] {
    //         x += 1;
    //     }
    // }
    // y = y - x;
    // println!("{} {}", x, y);
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
