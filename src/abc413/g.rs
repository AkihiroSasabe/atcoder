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
    // 2025-07-09 20:31-21:03 (32min, 提出)
    // 2025-07-09 21:03-21:30 (27min, debug in 電車)
    // 2025-07-09 23:00-24:43 (27min, debug, テストケースを書いたり。UFTの結合前にある2つの根を適切に処理しないといけなかった。)
    // Total: 86min
    
    let (h, w, k, r, c) = input();
    let res = solve(h, w, k, r.clone(), c.clone());
    // let res_bf = solve_bf(h, w, k, r, c);
    // assert!(res == res_bf);

    // let mut count = 0;
    // loop {
    //     println!("count = {} ----", count);
    //     let (h, w, k, r, c) = input_random();
    //     let res = solve(h, w, k, r.clone(), c.clone());
    //     let res_bf = solve_bf(h, w, k, r, c);
    //     assert!(res == res_bf);
    //     count += 1;
    // }

}

fn input_random() -> (usize, usize, usize, Vec<usize>, Vec<usize>) {
    let h = rand::thread_rng().gen_range(2..=5);
    let w = rand::thread_rng().gen_range(2..=5);
    let k = rand::thread_rng().gen_range(1..=min(h*w / 2, 5));
    println!("{} {} {}", h, w, k);
    let mut r = vec![];
    let mut c = vec![];
    let mut set = HashSet::new();
    while set.len() < k {
        let ri = rand::thread_rng().gen_range(0..h);
        let ci = rand::thread_rng().gen_range(0..w);
        if set.contains(&(ri, ci)) {continue;}
        if ri == 0 && ci == 0 {continue;}
        if ri == h-1 && ci == w-1 {continue;}
        set.insert((ri, ci));
        r.push(ri);
        c.push(ci);
    }
    for i in 0..k {
        println!("{} {}", r[i] + 1, c[i] + 1);
    }
    return (h, w, k, r, c)
}



fn input() -> (usize, usize, usize, Vec<usize>, Vec<usize>) {
    input! {
        h: usize,
        w: usize,
        k: usize,
    }
    let mut r = vec![];
    let mut c = vec![];
    for i in 0..k {
        input!{
            ri: Usize1,
            ci: Usize1,
        }
        r.push(ri);
        c.push(ci);
    }

    return (h, w, k, r, c)

}

fn solve(h: usize, w: usize, k: usize, r: Vec<usize>, c: Vec<usize>) -> bool {
    
    // 以下のケースでバグる
// 3 3 4
// 3 1
// 2 3
// 1 3
// 2 2
// Yes
// No

    let mut set = BTreeMap::new();
    let mut poss: Vec<(usize, usize, usize, usize)> = vec![(0,0,0,0); k];
    for i in 0..k {
        let ri = r[i];
        let ci = c[i];
        set.insert((ri, ci), i);
        poss[i] = (ri, ci, ri, ci);
    }
    // // 右、上、左、下
    // let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    // let dir_y: Vec<isize> = vec![0, -1, 0, 1];

    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0, 1, 1, -1, -1];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1, -1, 1, -1, 1];
    let mut uft = UnionFindTree::new(k);

    for v in 0..k  {
        let y = r[v];
        let x = c[v];

        for i in 0..dir_y.len() {
            let ny = dir_y[i] + y as isize;
            let nx = dir_x[i] + x as isize;
            if ny < 0 || (h as isize) <= ny || nx < 0 || (w as isize) <= nx {continue}
            let ny = ny as usize;
            let nx = nx as usize;
            if let Some(&nv) = set.get(&(ny, nx)) {
                // println!("v = {}, nv = {}", v, nv);
                if uft.issame(nv, v) {continue}

                let r = uft.root(v);
                let nr = uft.root(nv);
                uft.unite(nv, v);

                let mut y_min = min(poss[r].0, poss[nr].0);

                let mut x_min = min(poss[r].1, poss[nr].1);

                let mut y_max = max(poss[r].2, poss[nr].2);

                let mut x_max = max(poss[r].3, poss[nr].3);

                poss[r] = (y_min, x_min, y_max, x_max);
                poss[nr] = (y_min, x_min, y_max, x_max);
            }
        }
    }

    for i in 0..k {
        let r = uft.root(i);
        // let (y_min, x_min, y_max, x_max) = poss[i];
        let (y_min, x_min, y_max, x_max) = poss[r];
        // println!("(y_min, x_min, y_max, x_max) = {:?}", (y_min, x_min, y_max, x_max));
        if (y_max == (h - 1) || x_min == 0) && (y_min == 0 || x_max == (w - 1)) {
            println!("No");
            return false;
        }
    }
    println!("Yes");
    return true;


}

fn solve_bf(h: usize, w: usize, k: usize, r: Vec<usize>, c: Vec<usize>) -> bool {

    let mut img = vec![vec![false; w]; h];
    for i in 0..k {
        let y = r[i];
        let x = c[i];
        img[y][x] = true;
    }

    let mut graph = vec![vec![]; h * w];
    // 右、上、左、下
    let dir_x: Vec<isize> = vec![1, 0, -1, 0];
    let dir_y: Vec<isize> = vec![0, -1, 0, 1];
    for y in 0..h {
        for x in 0..w {
            if img[y][x] {continue}
            let v = y * w + x;
            for i in 0..4 {
                let ny = y as isize + dir_y[i];
                let nx = x as isize + dir_x[i];
                if ny < 0 || (h as isize) <= ny || nx < 0 || (w as isize) <= nx {continue}
                let ny = ny as usize;
                let nx = nx as usize;
                if img[ny][nx] {continue}
                let nv = ny * w + nx;
                graph[v].push(nv);
            }
        }
    }

    fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
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

    let dists = bfs(0,&graph);
    if dists[h*w-1] == 1_000_000_000_000_000_000 {
        println!("No");
        return false;
    } else {
        println!("Yes");
        return true;
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
