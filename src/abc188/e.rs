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
    // 2024-04-27 11:03-11:15 (12min)
    // 12:21-13:09 (48min)
    // 2024-04-28 15:39-16:27 (48min)
    input! {
        n: usize,
        m: usize,
        a: [isize;n],
    }
    // solve_mine(n, m, a);
    solve_dp_dag(n, m, a); // 自力でも解けたが、模範解答のDAGのDPの解法の方がスマート
}

fn solve_dp_dag(n: usize, m: usize, a: Vec<isize>) {
    let mut graph  =vec![vec![]; n];
    let mut in_degrees = vec![0; n];
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        input!{
            xi: usize,
            yi: usize,
        }
        graph[xi - 1].push(yi - 1);
        in_degrees[yi-1] += 1;
        uft.unite(xi-1, yi-1);
    }

    let sorted = topological_sort(&graph, &mut in_degrees);

    // dp[v] := その時点で買える最安値
    let INF= 1<< 60;
    let mut dp = vec![INF; n];
    for v in sorted {
        for &nv in graph[v].iter() {
            dp[nv] = min(dp[nv], a[v]);
            dp[nv] = min(dp[nv], dp[v]);
        }
    }

    let mut ans = - INF;
    for v in 0..n {
        // vで売るとき
        ans = max(ans, a[v] - dp[v]);
    }
    println!("{}", ans);
}

fn solve_mine(n: usize, m: usize, a: Vec<isize>) {
    // もっとも安いA[i]と、
    // もっとも高いA[j]を、選んでA[i] - A[j]を求めればいい。 ただし、i < j

    // トポロジカルソート
    // DAGであることは、Xi < Yi により保証済み

    let mut graph  =vec![vec![]; n];
    let mut in_degrees = vec![0; n];
    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        input!{
            xi: usize,
            yi: usize,
        }
        graph[xi - 1].push(yi - 1);
        in_degrees[yi-1] += 1;
        uft.unite(xi-1, yi-1);
    }

    let mut inds: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    for i in 0..n {
        let r = uft.root(i);
        inds.entry(r).or_insert(BTreeMap::new()).insert(i, in_degrees[i]);
    }

    // 連結成分毎に処理する。
    let mut ans = - (1 << 60);
    for (_root, ind) in inds.iter_mut() {
        let temp = bfs_topological(&graph, ind, &a);
        ans = max(ans, temp);
    }
    println!("{}", ans);


}

// トポロジカルソート(入次数が0のものからリストに入れていく)
fn bfs_topological(graph: &Vec<Vec<usize>>, ind: &mut BTreeMap<usize, usize>, a: &Vec<isize>) -> isize {
    let mut queue = VecDeque::new();
    let mut ans = - (1 << 60);

    let mut min_buy_hash = BTreeMap::new();
    for (v, deg) in ind.iter() {
        if *deg == 0 {
            queue.push_back((*v, a[*v]));
        }
        min_buy_hash.insert(*v, a[*v]);
    }

    while queue.len() != 0 {
        let (v, min_buy) = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            ans = max(ans, a[next_v]- min_buy);
            *ind.get_mut(&next_v).unwrap() -= 1;

            let next_min_buy = *min_buy_hash.get(&next_v).unwrap();
            *min_buy_hash.get_mut(&next_v).unwrap() = min(min_buy, next_min_buy);

            // 入次数が
            if *ind.get(&next_v).unwrap() == 0 {
                queue.push_back((next_v, *min_buy_hash.get(&next_v).unwrap()));
            }
        }
    }
    return ans
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



// トポロジカルソート(入次数が0のものからリストに入れていく)
fn topological_sort(graph: &Vec<Vec<usize>>, in_degrees: &mut Vec<usize>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut sorted_list = vec![];
    for i in 0..graph.len() {
        if in_degrees[i] == 0 {
            queue.push_back(i);
            sorted_list.push(i);
        }
    }
    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let next_v = graph[v][i];
            in_degrees[next_v] -= 1;
            // 入次数が
            if in_degrees[next_v] == 0 {
                queue.push_back(next_v);
                sorted_list.push(next_v);
            }
        }
    }
    return sorted_list
}