#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        mut n: usize,
        m: usize,
        mut x: [usize; n]
    }
    if n == m {
        println!("0");
        return;
    }
    x.sort();
    let mut diffs = BinaryHeap::new();

    // [3,5,9,10,20] の場合
    // 3-5, 5-9, 9-10,10-20 の差分

    for i in 0..n-1 {
        let diff = x[i+1] - x[i];
        diffs.push(Reverse((diff)));
    }

    let mut ans = 0;

    while n > m {
        if let Some(Reverse(diff)) = diffs.pop() {
            ans += diff;
            n -= 1;
        }
    }
    println!("{}", ans);





    // let mut heap= BinaryHeap::new();
    // let mut uft = UnionFindTree::new(n);
    // for i in 0..n-1 {
    //     let diff = x[i+1] - x[i];
    //     heap.push(Reverse((diff, i, i+1)));
    // }

    // let mut sets = vec![BTreeSet::new(); n];
    // for i in 0..n {
    //     if i != 0 {
    //         sets[i].insert(i-1);
    //     }
    //     if i != n-1 {
    //         sets[i].insert(i+1);
    //     }
    // }

    // let mut num = n;
    // loop {
    //     if num == m {break}
    //     if let Some(Reverse((diff, v, nv))) = heap.pop() {
    //         if uft.issame(v, nv) {continue}

    //         let rv = uft.root(v);
    //         let rnv = uft.root(nv);

    //         let left = *sets[rv].iter().next().unwrap();
    //         let right = *sets[rnv].iter().rev().next().unwrap();
    //         let n_diff = x[right] - x[left];
    //         uft.unite(v, nv);

    //         let r = uft.root(v);
    //         sets[r].insert(left);
    //         sets[r].insert(right);

    //         heap.push(Reverse((n_diff, left, right)));
    //         num -= 1;
    //     }
    // }

    // let mut ans = 0;
    // let mut now = 0;
    // let mut dist = 0;
    // for i in 0..n {
    //     // println!("i = {:?} ---- ", i);
    //     if uft.issame(now, i) {
    //         dist = x[i] - x[now];
    //     }
    //     else {

    //         // println!("dist = {:?}", dist);
    //         ans += dist;
    //         dist = 0;
    //         now = i;
    //     }
    // }
    // println!("{}", ans);



    // // めぐる式二分探索
    // // 関数じゃなくて、クロージャーを使うと、引数を少なく出来る。
    // let judge = |mid: usize| -> bool {
    //     let mut count = 0;
    //     let mut num_ok = 0;
    //     for i in 0..n {
    //         if i <= num_ok {

    //         }
    //     }



    //     return true
    // };
    // // fn judge(mid: usize) -> bool {
    // //    return true
    // // }
    
    // let mut ng = 0;
    // let mut ok = x[n-1] - x[0];
    // if judge(ng) {
    //     ok = ng;
    // }
    // while (ng as i128 - ok as i128).abs() > 1 {
    //     let mid = (ng + ok) / 2;
    //     let is_ok = judge(mid);
    //     if is_ok {
    //         ok = mid;
    //     }
    //     else {
    //         ng = mid;
    //     }
    // }
    // // println!("{}", ok);

}


// Union-Find
// グループの管理（グループ同士の結合や、要素同士の所属か同じか判定）するのに便利なデータ構造
#[derive(Clone)]
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
    fn print_for_debug(&mut self) {
        let n = self.sizes.len();
        let mut lists = vec![vec![]; n];
        for v in 0..n {
            let r = self.root(v);
            lists[r].push(v);
        }
        println!("uft = {:?}", lists);
        // print!("uft: ");
        // for v in 0..n {
        //     print!("({}: {:?}), ", v, lists[v]);
        // }
        // println!("");
    }
}
