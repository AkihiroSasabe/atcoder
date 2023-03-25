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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut g2= vec![HashMap::new(); n];
    let mut uft = UnionFindTree::new(n);
    let mut in_degrees = vec![0; n];
    let mut out_degrees = vec![0; n];
    let mut in_out_hash = HashMap::new();
    for i in 0..m {
        input! {
            x_i: usize,
            y_i: usize,
        }
        uft.unite(x_i-1, y_i-1);
        let hash_num = x_i + y_i * 400_000;
        if !in_out_hash.contains_key(&hash_num) {
            in_degrees[y_i-1] += 1;
            out_degrees[x_i-1] += 1;
            in_out_hash.insert(hash_num, 0);
        }
        if !g2[x_i-1].contains_key(&(y_i-1)) {
            graph[x_i-1].push(y_i-1);
            g2[x_i-1].insert(y_i-1, 0);
        }
    }
    
    // 考察
    // ・これはok
    // ○--->○--->○--->○
    // |\  /      
    // | \/       
    // |  ○       
    // |/         
    // ○

    // ・これはng (一意に定まらない. BFSによるトポロジカルソート中に、入次数が0のものが複数ある. 58行目と60行目右の頂点)
    // ○--->○--->○--->○
    // |\  /     |
    // | \/      |
    // |  ○      |
    // |/        |
    // ○-------->○

    // ・これはng (出次数が0個のものが複数ある)
    // ○--->○
    //  \
    //   ○--->○

    // 連結成分の個数を数える
    let mut hash = HashMap::new();
    for i in 0..n {
        let root = uft.root(i);
        if !hash.contains_key(&root) {
            hash.insert(root, vec![i]);
        }
        else {
            (*hash.get_mut(&root).unwrap()).push(i);
        }
    }
    // 連結成分が複数ある (一意じゃなくなる)
    if hash.len() != 1 {
        println!("No");
    }
    // 連結成分が1つだけ
    else {
        let mut start_candidate = vec![];
        let mut end_candidate = vec![];
        for i in 0..n {
            if in_degrees[i] == 0 {
                start_candidate.push(i);
            }
            else if out_degrees[i] == 0 {
                end_candidate.push(i);
            }
        }
        if start_candidate.len() == 1 && end_candidate.len() == 1 {
            // BFSによるトポロジカルソート (入次数が0のものからQueueに入れていく)
            let mut queue = VecDeque::new();
            let mut indice = vec![start_candidate[0]];
            queue.push_back(start_candidate[0]);

            while queue.len() != 0 {
                let v = queue.pop_front().unwrap();
                
                for i in 0..graph[v].len() {
                    let next_v = graph[v][i];
                    in_degrees[next_v] -= 1;
                    // 入次数
                    if in_degrees[next_v] == 0 {
                        queue.push_back(next_v);
                        indice.push(next_v);
                    }
                }
                // 一意に定まらない (この1行は普通のBFSのトポロジカルソートには必要ない)
                if queue.len() > 1 {break}
            }
            // 一意に定まる
            if indice.len() == n {
                println!("Yes");
                let mut a = vec![0; n];
                for i in 0..n {
                    a[indice[i]] = i + 1;
                }
                for i in 0..n {
                    print!("{} ", a[i]);
                }
            }
            // 一意に定まらない
            else {
                println!("No");
            }
        }
        // 入次数==0または出次数==0が複数個ある
        else {
            println!("No");
        }
    }
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