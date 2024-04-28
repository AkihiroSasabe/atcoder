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
        mut s: [Chars; h],
    }
    let mut magnets = vec![];
    let mut uft = UnionFindTree::new(h*w);

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                magnets.push((i, j));
            }
        }
    }


    let mut jun = BTreeSet::new();
    for &(y, x) in magnets.iter() {
        // 右、上、左、下
        let dir_x: Vec<isize> = vec![1, 0, -1, 0];
        let dir_y: Vec<isize> = vec![0, -1, 0, 1];
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;

            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if s[ny][nx] == '#' {continue}
            s[ny][nx] = '+';
            jun.insert((ny, nx));
        }
    }

    // println!("------------------");
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{}", s[y][x]);
    //     }
    //     println!("");
    // }
    
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' || s[y][x] == '+' {continue}
            let ny = y + 1;
            let nx = x + 1;

            if ny < h {
                if s[ny][x] == '.' {
                    uft.unite(w*y + x, w * ny + x);
                }
            }
            
            if nx < w {
                if s[y][nx] == '.' {
                    uft.unite(w*y + x, w * y + nx);
                }
            }
        }
    }


    let mut indeg  = vec![0; h*w];
    for &(y, x) in jun.iter() {
        // 右、上、左、下
        let dir_x: Vec<isize> = vec![1, 0, -1, 0];
        let dir_y: Vec<isize> = vec![0, -1, 0, 1];
        let mut set = BTreeSet::new();
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;

            if ny < 0 || s.len() as isize <= ny || nx < 0 || s[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            
            if set.contains(&uft.root(ny * w + nx)) {continue}
            if s[ny][nx] == '.' {
                indeg[uft.root(ny * w + nx)] += 1;
                set.insert(uft.root(ny * w + nx));
            }
        }
        // println!("y={}, x={}", y, x);
        // println!("set = {:?}", set);
    }

    let mut ans = 0;
    for i in 0..h*w {
        let r = uft.root(i);
        let r_size = uft.size(r);

        // println!("i={}, y = {}, x = {}", i, i / w, i % w);
        // println!("r_size = {:?}", r_size);
        // println!("indeg[r] = {:?}", indeg[r]);
        let temp = indeg[r] + r_size;
        // println!("temp = {:?}", temp);
        ans = max(ans, temp);
    }
    println!("{}", ans);


    // for y in 0..h {
    //     for x in 0..w {

    //     }
    // }

    






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
