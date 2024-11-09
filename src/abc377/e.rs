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
use rand::Rng;
fn main() {
    // 2024-10-26 21:44-22:40 (56min)
    // 2024-10-28 12:24-12:51 (27min, 解説読んだ。)
    // 参考
    // https://x.com/kyopro_friends/status/1850174871067865548 のグラフを書いて、実際に実験すると、ABC367Eの2^kの移動だと気づく。
    // https://x.com/nouka28/status/1850175280125800602 代数的には、 P[K](i)=P[K-1](P[K-1](i))=P[K-1]^2(i) で考えるとわかりそう。
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }
    // solve_my_real_time_wa(n, k, p);
    solve_ac(n, k, p);
}

fn solve_ac(n: usize, k: usize, mut p: Vec<usize>) {
    // P[K](i)=P[K-1](P[K-1](i))=P[K-1]^2(i)
    
    // i=0,1,..,n-1の順列P があるとき、
    // i -> p[i] (i=0,1,..,n-1) のn本のエッジ からなるグラフは、必ず何個かの閉路で構成される。
    // 理由: 全ての頂点が、入次数1, 出次数1 なので。

    // 入力例1
    // 6 3
    // 5 6 3 1 2 4
    // 
    // この場合、 i -> p[i] のグラフは、以下のような2個の閉路となる。
    // 閉路1: 1 -> 5 -> 2 -> 6 -> 4 -> 1
    // 閉路2: 3 <-> 3
    // P[K](i)  := K回の操作を終えたあと、最初のiの最終値とする。
    // K = 0 のとき、P[0] = 5,6,3,1,2,4
    // K = 1 のとき、P[1] = 2,4,3,5,6,1
    // K = 2 のとき、P[2] = 4,5,3,6,1,2
    // K = 3 のとき、P[3] = 6,1,3,2,4,5
    // となっていることから、P[k][i] = i からエッジに沿って2^k回移動した頂点であると気づける。（グラフを描くと、気付ける）
    // あとは、ダブリングで解くだけ

    // 2 ** k 歩先
    let mut uft = UnionFindTree::new(n);
    for i in 0..n {
        p[i] -= 1;
        uft.unite(i, p[i]);
    }

    // p2[x][i] := 初期位置iから、2^x回移動した先の頂点
    let mut p2 = vec![vec![0; n]; 64];
    for i in 0..n {
        p2[0][i] = p[i];
    }

    // ダブリング
    for x in 1..64 {
        for i in 0..n {
            p2[x][i] = p2[x-1][p2[x-1][i]];
        }
    }

    for i in 0..n {

        // 閉路の長さ
        let modulus = uft.size(i);

        // 移動回数 = 2^k % (閉路の長さ)
        let num = mod_pow(2, k, modulus);
        
        let mut pos = i;
        for x in 0..64 {
            if (1 << x) & num != 0 {
                pos = p2[x][pos];
            } 
        }
        print!("{} ", pos + 1);
    }

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



// コンテスト中に考えた誤答
fn solve_my_real_time_wa(n: usize, k: usize, mut p: Vec<usize>) {
    // let modulus: usize = 1_000_000_000 + 7;
    let modulus: usize = 2_147_483_647; // 約 2 * 10^9
    let modulus_1: usize = 1_000_000_007; // 約 10^9

    for i in 0..n {
        p[i] -= 1;
    }
    let mut p_origin = p.clone();

    // for _ in 0..10000 {
    // println!("p = {:?}", p);
    // for _ in 0..60 {

    let mut count = 0;
    let mut set = BTreeMap::new();
    let mut hash0 = 0;
    let mut hash1 = 0;
    let mut pow = 1;
    let mut pow_1 = 1;
    for i in 0..n {
        hash0 += pow * p[i] % modulus;
        hash1 += pow_1 * p[i] % modulus_1;
        pow *= 10;
        pow_1 *= 10;
        pow %= modulus;
        pow_1 %= modulus_1;
        set.insert((hash0, hash1), count);
    }

    loop {
        count += 1;
        let mut pre_p = p.clone();
        for i in 0..n {
            p[i] = pre_p[pre_p[i]];
        }
        hash0 = 0;
        hash1 = 0;
        pow = 1;
        pow_1 = 1;
        for ii in 0..n {
            hash0 += pow * p[ii] % modulus;
            hash1 += pow_1 * p[ii] % modulus_1;
            pow *= 10;
            pow_1 *= 10;
            pow %= modulus;
            pow_1 %= modulus_1;
        }
        if set.contains_key(&(hash0, hash1)) {
            break
        }
        else {
            // set.insert(p.clone(), count);
            
            set.insert((hash0, hash1), count);
        }
        // println!("p = {:?}", p);
    }
    let mut hash0 = 0;
    let mut hash1 = 0;
    let mut pow = 1;
    let mut pow_1 = 1;
    for i in 0..n {
        hash0 += pow * p[i] % modulus;
        hash1 += pow_1 * p[i] % modulus_1;
        pow *= 10;
        pow_1 *= 10;
        pow %= modulus;
        pow_1 %= modulus_1;
    }
    let pre = *set.get(&(hash0, hash1)).unwrap();
    let shuki = count - pre;

    let mut num = 0;
    if k <= pre {
        num = k
    }
    else {
        let diff = k - pre;
        num = pre + diff % shuki;
    }
    // println!("num = {:?}", num);

    for _ in 0..num {
        let mut pre_p = p_origin.clone();
        for i in 0..n {
            p_origin[i] = pre_p[pre_p[i]];
        }
    }
    for i in 0..n {
        print!("{} ", p_origin[i] + 1);
    }
    return

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
