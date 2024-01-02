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
    // 2024-01-02 17:14-17:56 (42min)
    // 2024-01-02 18:48-
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    const MODULUS: usize = 998_244_353;

    // # 緑
    let mut uft = UnionFindTree::new(h*w);
    // 右、下、左、上
    let dir_y = [0, 1, 0, -1];
    let dir_x = [1, 0, -1, 0];

    for y in 0..(h as isize) {
        for x in 0..(w as isize) {
            for d in 0..4 {
                let dx = dir_x[d];
                let dy = dir_y[d];
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {continue}
                let nx = nx as usize;
                let ny = ny as usize;

                if s[y as usize][x as usize] == '#' && s[ny][nx]  == '#' {
                    let v0 = y as usize * w  + x as usize;
                    let v1 = ny as usize * w + nx as usize; 
                    uft.unite(v0, v1);
                }
            }
        }
    }
    let mut hash = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if s[y][x] != '#' {continue}
            let r = uft.root(y * w + x);
            hash.insert(r);
        }
    }
    // 連結成分の個数
    let num_connects = hash.len();
    // println!("num_connects = {:?}", num_connects);

    let mut bunbo = 0;
    let mut bunsi = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] != '#' {
                bunbo += 1;
                let mut hm = HashSet::new();
                for d in 0..4 {
                    let dx = dir_x[d];
                    let dy = dir_y[d];
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
    
                    if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {continue}
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[ny][nx] != '#' {continue}
                    hm.insert(uft.root(ny * w + nx));
                }
                // println!("----hm = {:?}, y,x = {},{}", hm, y, x);
                // println!("hm.len() = {:?}", hm.len());
                let diff = num_connects + 1 - hm.len();

                // println!("diff = {:?}", diff);
                bunsi += diff;
            }
        }
    }
    // println!("bunsi = {:?}", bunsi);
    // println!("bunbo = {:?}", bunbo);
    let ans = mod_dev(bunsi, bunbo, MODULUS);
    println!("{}", ans);

}



// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a % modulo * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a % modulo, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    base %= modulo;
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
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
