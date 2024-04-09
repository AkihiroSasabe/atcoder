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
    // 2024-04-06 21:14-22:17 (63min)
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }
    input! {
        n: usize,
    }
    let mut r = vec![];
    let mut c = vec![];
    let mut e = vec![];
    for i in 0..n {
        input!{
            ri: usize,
            ci: usize,
            ei: usize,
        }
        r.push(ri-1);
        c.push(ci-1);
        e.push(ei);
    }
    let mut img = vec![vec![0; w]; h]; // 薬の大きさ
    let INF = 1 << 60;
    let mut aa = vec![vec![INF-1; w]; h]; // INF: 壁, INF-1: 空き, それ以外: 薬のindex
    for i in 0..n {
        let ri = r[i];
        let ci = c[i];
        let ei = e[i];
        img[ri][ci] = ei;
        aa[ri][ci] = i;
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut tx = 0;
    let mut ty = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                sy = i;
                sx = j;
            }
            if a[i][j] == 'T' {
                ty = i;
                tx = j;
            }
            if a[i][j] == '#' {
                aa[i][j] = INF;
            }
        }
    }

    let mut graph = vec![vec![]; n+2]; // nがs, 　n+1がt
    let si = aa[sy][sx];
    if si >= INF - 1 {
        println!("No");
        return;
    }
    else {
        graph[n].push(aa[sy][sx]);
    }


    let si = n;
    let ti = n + 1;

    // let mut uft = UnionFindTree::new(n+1);
    for i in 0..n {
        let y = r[i];
        let x = c[i];
        let ei = e[i];
        // start
        let distmap = bfs(y, x, &aa);
        for j in 0..n {
            let yj = r[j];
            let xj = c[j];
            if distmap[yj][xj] <= ei {
                // uft.unite(i, j);
                graph[i].push(j);
            }
        }
        if distmap[sy][sx] <= ei {
            // uft.unite(i, n);
            graph[i].push(n);
        }
        if distmap[ty][tx] <= ei {
            // uft.unite(i, n+1);
            graph[i].push(n+1);
        }
    }

    let dist2 = bfs2(si, &graph);
    if dist2[ti] != 1_000_000_000_000_000_000 {
        println!("Yes");
    }
    else {
        println!("No");
    }



}


fn bfs2(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[v0] = 0;
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    return distance
}

// fn dfs(start: usize, y: usize, x: usize, ene: usize, img: &Vec<Vec<usize>>, aa: &Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>, uft: &mut UnionFindTree) {
//     seen[y][x] = true;
//     // 右、上、左、下
//     let dir_x: Vec<isize> = vec![1, 0, -1, 0];
//     let dir_y: Vec<isize> = vec![0, -1, 0, 1];
//     for i in 0..dir_y.len() {
//         let ny = dir_y[i] + y as isize;
//         let nx = dir_x[i] + x as isize;
//         if ny < 0 || img.len() as isize <= ny || nx < 0 || img[0].len() as isize <= nx {continue}
//         let ny = ny as usize;
//         let nx = nx as usize;
//         if seen[ny][nx] {continue}
//         if img[ny][nx] > 0 {
//             uft.unite(start, img[ny][nx]);
//         }
//         if ene == 0 {continue}
//         dfs(start, ny, nx, ene-1, img, aa, seen, uft);
//     }
    
// }

fn bfs(sy: usize, sx: usize, aa: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![vec![init_distance; aa[0].len()]; aa.len()];
    // let mut distance = vec![init_distance; graph.len()];
    distance[sy][sx] = 0;
    queue.push_back((sy, sx));
    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    let INF = 1 << 60;

    while queue.len() > 0 {
        let (y, x) = queue.pop_front().unwrap();
        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || aa.len() as isize <= ny || nx < 0 || aa[0].len() as isize <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            // 壁
            if aa[ny][nx] == INF {continue}
            if distance[ny][nx] != init_distance {continue}
            distance[ny][nx] = distance[y][x] + 1;
            queue.push_back((ny, nx));
        }
    }
    return distance
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
