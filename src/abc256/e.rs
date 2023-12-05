use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    // 2023-12-04 20:50-20:55 (5min)
    // 2023-12-04 22:11-23:09 (58min)
    // total 63min
    input! {
        n: usize,
        mut x: [usize; n],
        mut c: [isize; n],
    }

    // 全てのコストを事前に足し合わせる。そこから何を引くかを後で考える。
    let mut cost_sum = 0;
    for i in 0..n {
        x[i] -= 1;
        cost_sum += c[i];
    }

    // 頂点N個で、2N本のエッジを持つグラフを作成する。
    let mut graph = vec![HashMap::new(); n];
    for v in 0..n {
        // 嫌いな人x[i] -> 人i へのエッジは、コストがかかる繋ぎ方。
        // ただしコストは事前に加算済みなので、コストのかかる繋ぎ方の場合でも重みは0にしておく
        *graph[x[v]].entry(v).or_insert(0) += 0;
        
        // 逆辺を選ぶことは、その辺を外すことと同義なので、重みは負のコスト。
        *graph[v].entry(x[v]).or_insert(0) -= c[v];
    }

    let mut edges = vec![];
    let mut costs = vec![];
    for i in 0..n {
        for (v, c) in graph[i].iter() {
            costs.push((*c, edges.len()));
            edges.push((*v, i));
        }
    }

    // クラスカル法っぽくやる (コストが低い順に、エッジを選択していく。ループはできないようにUFTで管理する)
    costs.sort();
    let mut uft = UnionFindTree::new(n);

    for i in 0..edges.len() {
        let cost = costs[i].0;
        let ind = costs[i].1;

        let v0 = edges[ind].0;
        let v1 = edges[ind].1;

        if uft.issame(v0, v1) {
            continue
        }
        cost_sum += cost;
        uft.unite(v0, v1);
    }
    println!("{}", cost_sum);
    
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
