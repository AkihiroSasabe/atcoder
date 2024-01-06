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
    // 2024-01-06 17:47-18:41 (54min)
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut uft = UnionFindTree::new(n);
    let mut degree = vec![0; n];
    for i in 0..m {
        input! {
            u_i: usize,
            v_i: usize,
        }
        let u = u_i - 1;
        let v = v_i - 1;
        graph[u].push(v);
        graph[v].push(u);
        uft.unite(u, v);
        degree[u] += 1;
        degree[v] += 1;
    }
    let modulus: usize = 998_244_353;

    // 全頂点の出次数 == 1 であることは、図を描いて考察すると
    // 全ての連結成分が、なもりグラフ(連結で、|V|=|E|なグラフ)であることと、同じであることが分かる。
    // まずは、全連結成分がなもりグラフか判定し、そうだった場合に2^(連結成分の個数)を求めれば良い。
    
    // なもりグラフか判定 (連結成分iについて、|Vi| == |Ei|が示せるだけで十分だった。)
    // 次数が1の頂点を削除していって、最終的に、次数が3以上のものがあれば、失格。
    let mut hash = HashMap::new();
    for i in 0..n {
        hash.entry(uft.root(i)).or_insert(vec![]).push(i);
    }

    // 入次数が1以下の頂点を、BFSで枝刈りしていく。
    let mut deque = std::collections::VecDeque::new();
    for v in 0..n {
        if degree[v] <= 1 {
            // 入次数が1以下のもの
            deque.push_back(v);
        }
    }
    let mut seen = vec![false; n];
    while deque.len() != 0 {
        let v = deque.pop_front().unwrap();
        seen[v] = true;
        for &nv in graph[v].iter() {
            if seen[nv] {continue}
            degree[nv] -= 1;
            if degree[nv] <= 1 {
                deque.push_back(nv);
            }
        }
    }

    // 枝刈り後に残ったグラフのうち、次数が3以上のやつが、残っていたら環が2個あるので失格。
    for i in 0..n {
        if degree[i] >= 3 {
            println!("0");
            return;
        }
    }


    // 枝刈り後に、全部消滅してしまう連結グラフがあったら、環が0個しかないので、失格。
    for (root, vs) in hash.iter() {
        let mut flag = false;
        for v in vs {
            if degree[*v] == 2 {
                flag = true;
            }
        }
        if !flag {
            println!("0");
            return;
        }
    }

    // なもりグラフの数 ^ 2 が答えだろう。
    let mut ans = 1;
    for (_, _) in hash.iter() {
        ans *= 2;
        ans %= modulus;
    }
    println!("{}", ans);



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
