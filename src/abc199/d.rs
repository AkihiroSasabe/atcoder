#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::{concat, Itertools};
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
    // 2025-06-26 20:16-21:20 (1h4m, 方針は思い浮かんでいた)
    // 2025-06-26 23:32-26:10 (2h38m, debug, num_visitのデバッグに一番時間掛かった。あとvからのエッジのループと、状態のループの順番とか)
    // 3h42m 
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for i in 0..m {
        input!{
            ui: Usize1,
            vi: Usize1,
        }
        graph[ui].push(vi);
        graph[vi].push(ui);
        edges.push((ui, vi));
    }

    let mut uft = UnionFindTree::new(n);
    for i in 0..m {
        let (u,v) = edges[i];
        uft.unite(u, v);
    }
    let mut set = HashMap::new();
    for i in 0..n {
        let r = uft.root(i);
        set.entry(r).or_insert(vec![]).push(i);
    }
    let mut connects = vec![];
    for (root, mut vs) in set {
        connects.push(vs);
    }

    // 23:47-
    // 計算量: 
    // 結局、色の塗り方の選択肢は、
    // 次の色は、自分以外の2通りにしかならないので、
    // 2^(n-1) <= 2^19 = 524_288
    // 程度の状態しかない。
    
    fn dfs(v: usize, g: &Vec<Vec<usize>>, states: &mut Vec<usize>, num_visit: &mut usize, max_num_visit: usize, ans: &mut usize) {
        // 色に矛盾がないかチェック
        let mut ok_states = vec![];
        for &state in states.iter() {
            let mut is_ok = true;
            for i in 0..g[v].len() {
                let nv = g[v][i];
                let pow_v = (1 << v) * (1 << v);
                let pow_nv = (1 << nv) * (1 << nv);

                let c_v = (state / pow_v) % 4;
                let c_nv = (state / pow_nv) % 4;
                if c_v == c_nv {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                ok_states.push(state);
            }
        }
        swap(&mut ok_states, states);
        if states.len() == 0 {
            return;
        }
        *num_visit += 1;
        if *num_visit == max_num_visit {
            *ans = states.len();
            return;
        }

        // ok_states = states.clone();

        // 次の状態へ。
        for i in 0..g[v].len() {
            let nv = g[v][i];
            let pow_v = (1 << v) * (1 << v);
            let pow_nv = (1 << nv) * (1 << nv);
            // 訪問済み
            if (states[0] / pow_nv) % 4 != 3 {continue}
            let mut next_states = vec![];
            for &state in states.iter() {
                let color_v = (state / pow_v) % 4;
                let color_nv_1 = (color_v + 1) % 3;
                let color_nv_2 = (color_v + 2) % 3;
                let n_state1 = state + color_nv_1 * pow_nv - 3 * pow_nv;
                let n_state2 = state + color_nv_2 * pow_nv - 3 * pow_nv;
                next_states.push(n_state1);
                next_states.push(n_state2);
            }
            swap(&mut next_states, states);
            dfs(nv, g, states, num_visit, max_num_visit, ans);
            if states.len() == 0 {return}
        }

    }

    // >>> 4 ** 20
    // 1_099_511_627_776
    let mut ans = 1;
    let init_state = (1 << n) * (1 << n) - 1;
    for vs in connects {
        let v0 = vs[0];
        let pow = (1 << v0) * (1 << v0);

        // v0 を 0 の色にする
        let state = init_state - pow * 3;
        let mut states = vec![state];
        let mut cand = 0;
        let mut num_visit = 0;
        dfs(v0, &graph, &mut states, &mut num_visit, vs.len(), &mut cand);
        ans = ans * 3 * cand;
    }
    println!("{}", ans);
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
