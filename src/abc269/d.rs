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
use superslice::*;
fn main() {
    input! {
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut xy_n = vec![vec![n; 2001]; 2001];
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize
        }
        x.push(x_i);
        y.push(y_i);
        xy_n[(y_i+1000) as usize][(x_i+1000) as usize] = i;
    }

    let mut uft = UnionFindTree::new(n);
    // let mut black = vec![vec![false; 2001]; 2001];

    for i in 0..n {
        let x_1 = x[i] - 1;
        let y_1 = y[i] - 1;
    
        let x_2 = x[i] - 1;
        let y_2 = y[i];
    
        let x_3 = x[i];
        let y_3 = y[i] - 1;
    
        let x_4 = x[i];
        let y_4 = y[i] + 1;
    
        let x_5 = x[i] + 1;
        let y_5 = y[i];
    
        let x_6 = x[i] + 1;
        let y_6 = y[i] + 1;
        

        if check(x_1, y_1) && xy_n[(y_1+1000) as usize][(x_1+1000) as usize] != n {
            uft.unite(i, xy_n[(y_1+1000) as usize][(x_1+1000) as usize]);            
        }
        if check(x_2, y_2) && xy_n[(y_2+1000) as usize][(x_2+1000) as usize] != n {
            uft.unite(i, xy_n[(y_2+1000) as usize][(x_2+1000) as usize]);            
        }
        if check(x_3, y_3) && xy_n[(y_3+1000) as usize][(x_3+1000) as usize] != n {
            uft.unite(i, xy_n[(y_3+1000) as usize][(x_3+1000) as usize]);            
        }
        if check(x_4, y_4) && xy_n[(y_4+1000) as usize][(x_4+1000) as usize] != n {
            uft.unite(i, xy_n[(y_4+1000) as usize][(x_4+1000) as usize]);            
        }
        if check(x_5, y_5) && xy_n[(y_5+1000) as usize][(x_5+1000) as usize] != n {
            uft.unite(i, xy_n[(y_5+1000) as usize][(x_5+1000) as usize]);            
        }
        if check(x_6, y_6) && xy_n[(y_6+1000) as usize][(x_6+1000) as usize] != n {
            uft.unite(i, xy_n[(y_6+1000) as usize][(x_6+1000) as usize]);            
        }
    }
    let mut ans_list = vec![0; n];
    for i in 0..n {
        let root = uft.root(i);
        ans_list[root] = 1;
    }
    let mut ans = 0;
    for i in 0..n {
        if ans_list[i] == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);

}

fn check(x: isize, y: isize) -> bool {
    return !(x < -1000 || y < - 1000 || 1000 < x || 1000 < y)
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
