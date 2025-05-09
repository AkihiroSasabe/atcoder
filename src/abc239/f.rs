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
use proconio::marker::{Chars, Usize1};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    // 2025-05-09 23:34-1:01 (1h27min)
    // 2025-05-09 1:24-01:30 (6min)
    // 1h33min
    // 【マージテク(union by size)、木】頂点vの次数がd[v]の木になるように、無向グラフに追加するべき辺を答える問題。
    // 連結成分ごとに、辺を追加可能な頂点とその個数を、setと heapとUFTで管理して条件に適合するか判定すればよい。
    // 追加可能な辺数が上位1位と2位の連結成分同士で、辺を追加して連結していく。
    // 連結の際には、辺を追加可能な頂点が少ない方から多い方へマージする。
    input! {
        n: usize,
        m: usize,
        mut d: [usize; n],
    }

    // sum := 次数の総和
    let mut sum = 0;
    for i in 0..n {
        sum += d[i];
    }
    // 最終的な辺の数は、N-1なので、次数は2N-2じゃないとNG
    if sum != 2*n-2 {
        println!("-1");
        return;
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
        }
        a.push(ai);
        b.push(bi);
        // 付与された次数以上に、辺が貼られていたらNG
        if d[ai] == 0 || d[bi] == 0 {
            println!("-1");
            return;
        }
        d[ai] -= 1;
        d[bi] -= 1;

        uft.unite(ai, bi);
    }

    // capacities[v] := 頂点vに連結した成分が持つ、辺を張ることが可能な頂点の集合(頂点v0がx本の頂点を張れるなら、v0をx個追加する。)
    // 連結成分同士で、重複してカウントはしない。(v0とv1が連結成分なら、uftのrootの方にだけ、辺を張れる頂点集合を格納し、他は空にしておく。)
    let mut capacities = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..d[i] {
            let r = uft.root(i);
            capacities[r].push(i);
        }
    }
    // heap には、(連結成分が追加で張れる辺の本数, 連結成分の根) を格納する。
    let mut heap = BinaryHeap::new();
    // seen[r] := 根rの連結成分を、heapに格納済みか?
    let mut seen = vec![false; n];
    for i in 0..n {
        let r = uft.root(i);
        if seen[r] {continue}
        seen[r] = true;
        if capacities[r].len() == 0 {continue}
        heap.push((capacities[r].len(), r));
    }
    // println!("heap = {:?}", heap);
    // println!("capacities = {:?}", capacities);

    // anss := 結合する頂点対を格納
    let mut anss = vec![];
    while heap.len() != 0 {
        // 張れる辺の個数が上位1番と2番の連結成分同士を合併させる。
        let ((num0, v0)) = heap.pop().unwrap();
        if let Some((num1, v1)) = heap.pop() {
            // println!("(v0, v1, num0, num1) = {:?}", (v0, v1, num0, num1));
            // num1 -> num0 へ
            let new_num = num1 + num0 - 2;

            // 辺を追加可能な頂点を、2つの連結成分から取り出す。
            let ans0 = capacities[v0].pop().unwrap();
            let ans1 = capacities[v1].pop().unwrap();
            anss.push((ans0, ans1));

            // 残った capacities を統合する (union by size で少ない方から、大きい方に移動させる。)
            let mut temp = vec![];
            for &vv in capacities[v1].iter() {
                temp.push(vv);
            }
            for vv in temp {
                capacities[v0].push(vv);
            }
            capacities[v1] = vec![];
            
            // v0 と v1 を結合する。
            uft.unite(v0, v1);
            let r = uft.root(v0);
            if r == v1 {
                // ルートがv1だったら、capacities[v0] の所有権を、v1に移す。
                capacities.swap(v0, v1);
            }
            if new_num >= 1 {
                heap.push((new_num, r));
            }
        }
        else {
            println!("-1");
            return;
        }
        // println!("capacities = {:?}", capacities);
    }

    if uft.size(0) != n {
        println!("-1");
        return;
    }
    for i in 0..n {
        if capacities[i].len() != 0 {
            println!("-1");
            return;
        }
    }

    for ans in anss {
        println!("{} {}", ans.0 + 1, ans.1 + 1);
    }
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
