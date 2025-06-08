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
    // 2025-06-08 9:44-10:35 (51min)
    input! {
        n: usize,
        q: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
    }

    let mut t = vec![];
    let mut xq = vec![];
    let mut yq = vec![];
    let mut uq = vec![];
    let mut vq = vec![];

    let mut num_v = n;
    for i in 0..q {
        input!{
            ti: usize,
        }
        t.push(ti);

        if ti == 1 {
            input!{
                ai: isize,
                bi: isize,
            }
            xq.push(ai);
            yq.push(bi);
            x.push(ai);
            y.push(bi);
            uq.push(0);
            vq.push(0);
            num_v += 1;
        }
        else if ti == 2 {
            xq.push(0);
            yq.push(0);
            uq.push(0);
            vq.push(0);
        }

        else if ti == 3 {
            input!{
                ui: Usize1,
                vi: Usize1,
            }
            xq.push(0);
            yq.push(0);
            uq.push(ui);
            vq.push(vi);
        }
    }

    let mut current_num_v = n;
    let mut uft = UnionFindTree::new(num_v);
    // let mut set = BTreeSet::new();
    let mut heap = BinaryHeap::new();

    for u in 0..n {
        for v in u+1..n {
            let dist = (x[u] - x[v]).abs() + (y[u] - y[v]).abs();
            // set.insert((dist, u, v));
            heap.push(Reverse((dist, u, v)));
        }
    }

    for i in 0..q {
        let ti = t[i];
        if ti == 3 {
            if uft.issame(uq[i], vq[i]) {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
        else if ti == 1 {
            // 頂点を追加
            for v0 in 0..current_num_v {
                let dist = (x[v0] - x[current_num_v]).abs() + (y[v0] - y[current_num_v]).abs();
                heap.push(Reverse((dist, v0, current_num_v)));
            }
            current_num_v += 1;

        }
        else if ti == 2 {
            // 連結成分同士の最短距離を print
            if uft.size(0) == current_num_v {
                // 連結成分が1個だけ
                println!("-1");
                continue
            }
            // 連結成分が2個以上
            // ポップし続ける。

            let mut is_first = true;
            let mut min_dist = 1_000_000_000_000_000;
            while let Some(Reverse((dist, ui, vi))) = heap.pop() {
                // 連結成分
                if uft.issame(ui, vi) {continue}

                // 非連結
                if is_first {
                    min_dist = dist;
                    uft.unite(ui, vi);
                    is_first = false;
                    println!("{}", dist);
                }
                else {
                    if min_dist < dist {
                        heap.push(Reverse((dist, ui, vi)));
                        break
                    }
                    else {
                        uft.unite(ui, vi);
                    }
                }
            }
        }

    }

}

// 正解
// No
// 1
// Yes
// 2
// No
// 3 (2になった。)
// Yes (Noになった)
// -1 (3になった)
// 0


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
