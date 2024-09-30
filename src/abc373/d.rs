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
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut uft = PotentialUnionFindTree::new(n);
    for i in 0..m {
        input!{
            ui: usize,
            vi: usize,
            wi: isize,
        }
        graph[ui-1].push((vi-1, wi));
        // graph[vi-1].push((ui-1, wi));
        uft.unite(vi-1, ui-1, wi);
    }

    for i in 0..n {
        let r = uft.root(i);
        let diff = uft.diff(i, r).unwrap();
        print!("{} ", diff);
    }


    
}

// Potential Union-Find-Tree (重み付きUFT): 
// 各ノードに根からのポテンシャルを定義し、ノード間の距離を管理できるように拡張したUFT
// ■普通のUFT
// unite(v0,v1)     頂点v0 を含むグループと 頂点v1 を含むグループをマージする
// issame(v0,v1)    頂点v0 と 頂点v1 が同じグループにいるかどうかを判定する
//
// ■重み付きUFT:
// unite(v0,v1,w)   potential(v0) - potential(v1) == w となるように結合する。以前の結合結果と整合性があるチェックし、矛盾がない場合だけ結合する。
// issame(v0,v1)    頂点v0 と 頂点v1 が同じグループにいるかどうかを判定する
// diff(v0,v1)      v0 と v1 とが同じグループにいるとき、ポテンシャルの差分 potential(v0) - potential(v1) を取得
// 
// 裁量
// ・xとyが既に同じグループにいるとき、unite(x,y,w)を呼んじゃうか? -> 自分のは呼ぶ。ポテンシャルに矛盾があればfalseを返す
// ・xとyが同じグループにいないときにdiff(x,y)を呼んじゃうか? -> 自分のはNoneを返す
// 実装方法と内部の処理の理解はけんちょん氏の記事が参考になる　https://qiita.com/drken/items/cce6fc5c579051e64fab
struct PotentialUnionFindTree {
    parents: Vec<usize>,    // 各頂点の属するグループ(根付き木)の親頂点の番号
    sizes: Vec<usize>,      // 各頂点の属するグループ(根付き木)のサイズ(頂点数)
    potentials: Vec<isize>   // 親頂点から見た子頂点のポテンシャル
}

impl PotentialUnionFindTree {
    fn new(n: usize) -> Self {
        // 各頂点が属するグループの根を格納
        let parents: Vec<usize> = (0..n).collect();

        // 各頂点が属するグループのサイズ(頂点数)を格納。※ただし、更新するのは根のインデックスだけで良い
        let sizes: Vec<usize> = vec![1; n];
        
        // 親頂点から見た子頂点のポテンシャル。
        // 初期状態では、各頂点は独立しているので、自分自身が親頂点であり、ポテンシャルは0
        let potentials: Vec<isize> = vec![0; n];

        return PotentialUnionFindTree {parents, sizes, potentials}
    }

    // 根を求める。経路圧縮により計算量を削減
    fn root(&mut self, v: usize) -> usize {
        if self.parents[v] == v {
            // 頂点vが頂点vの親pと一致したとき、vがそのグループの根である。
            return v
        }
        else {
            // 頂点vの根 = 親pの根 なので、再帰的に根を求められる
            let r = self.root(self.parents[v]);

            // vの親を根に張り替える場合、ポテンシャルの原点も、古い親から新しい親に合わせる必要がある。
            // 古い親自身のポテンシャルを、オフセットすればいい (根のポテンシャルは0なことに注意)
            self.potentials[v] += self.potentials[self.parents[v]];

            // 経路圧縮 (親を根に張り替える。)
            self.parents[v] = r;
            return self.parents[v]
        }
    }

    // 頂点vのポテンシャルを取得
    fn potential(&mut self, v: usize) -> isize {
        // 経路圧縮 (vの親を、確実に最新の根に張替える。)
        self.root(v);
        return self.potentials[v]
    }
    // 頂点v0とv1のポテンシャル差: P(v0) - P(v1) を取得
    fn diff(&mut self, v0: usize, v1: usize) -> Option<isize> {
        if self.issame(v0, v1) {
            return Some(self.potential(v0) - self.potential(v1))
        }
        else {
            // v0とv1が違うグループにいるときは、ポテンシャル差が分からないのでNoneで返す
            return None
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

    // potential(v0) - potential(v1) = w となるように 併合 する。(以前の併合結果との整合性をチェックし、矛盾があったら何もしない)
    // v0を含むグループと、v1を含むグループとを併合する。Union by sizeで計算量削減。
    fn unite(&mut self, v0: usize, v1: usize, w: isize) -> bool {
        // 既に同じグループであれば何もしない
        let r0 = self.root(v0);
        let r1 = self.root(v1);
        
        // r1 からみた、 r0 のポテンシャルの高さ
        let potential_r0_from_r1 = self.potential(v1) + w - self.potential(v0); // 絵を書くとこの式の意味がよく分かる
        if r0 == r1 {
            // v0 と v1 が既に繋がっている場合は何もしない
            // 以前の併合結果と矛盾する場合は、整合性を false で返す
            return potential_r0_from_r1 == 0
        }

        // 統合後の状態
        let child: usize;
        let parent: usize;
        let child_potential: isize; // 結合後の新しい根から見た、取り込まれたグループの旧根のポテンシャル
        // Union by sizeにより、サイズが小さいグループを、サイズが大きいグループに併合する
        if self.size(r0) <= self.size(r1) {
            child = r0;
            parent = r1;
            child_potential = potential_r0_from_r1;
        }
        else {
            child = r1;
            parent = r0;
            child_potential = - potential_r0_from_r1;
        }
        self.sizes[parent] += self.size(child);
        self.parents[child] = parent;
        self.potentials[child] = child_potential;

        // 整合性のある併合ができた場合は、trueで返す
        return true
    }
}
