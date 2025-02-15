#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-14 18:56-21:00 (2h4min)
    // 2025-02-15 11:28-13:37 (2h9min)
    // 4h13min
    input! {
        mut ax: isize, // チョップの軌道の端点
        mut ay: isize,
        mut bx: isize,
        mut by: isize,
        n: usize,
    }
    // 縦斬りのときは、90度回転させておく
    let ax2 = ax - ay;
    let ay2 = ax + ay;
    let bx2 = bx - by;
    let by2 = bx + by;

    let is_tate = ax == bx;
    if is_tate {
        ax = ax2;
        bx = bx2;
        ay = ay2;
        by = by2;
    }

    // 板の形
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        if !is_tate {
            x.push(xi);
            y.push(yi);
        }
        else {
            x.push(xi - yi);
            y.push(yi + yi);
        }
    }
    // 交点の個数を数える。
    // そして、x座標でソートする

    let mut crosses = vec![];
    let mut is_cut = vec![false; n];
    for i in 0..n {
        let j = (i + 1) % n;
        let x0 = x[i];
        let y0 = y[i];
        let x1 = x[j];
        let y1 = y[j];
        if let Some((yc,xc)) = get_cross_in_line_segment(y0,x0,y1,x1,ay,ax,by,bx) {
            crosses.push((xc, yc, i));
            is_cut[i] = true;
        }
    }
    
    crosses.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());

    // for i in 0..crosses.len() {
    //     println!("crosses[{i}] = {:?}", crosses[i]);
    // }


    let num_vertex = crosses.len() * 2 + n;
    let mut uft: UnionFindTree = UnionFindTree::new(num_vertex);
    // println!("num_vertex = {:?}", num_vertex);

    // 切断されていなければ頂点は繋いでおく。
    let mut cut_count = 0;
    for i in 0..n {
        if !is_cut[i] {
            let j = (i + 1) % n;
            uft.unite(i, j);
        }
    }

    // 切断点が、左側にあるやつから
    for i in 0..crosses.len() {
        // println!("i = {:?} ----------", i);
        let xc = crosses[i].0;
        let yc = crosses[i].1;
        let v = crosses[i].2;
        let nv = (v + 1) % n;

        let cut_up = n+i;
        let cut_down = n+i+crosses.len();
        // println!("(cut_up, cut_down) = {:?}", (cut_up, cut_down));

        if i % 2 == 1 {
            // 偶数番の切断点は、1個前の切断点と繋いで、切り口になる。

             // 上側の切断点同士の結合
            uft.unite(cut_up, n+i-1);

            // 下側の切断点同士の結合
            uft.unite(cut_down, n+i-1 + crosses.len());
        }

        // vが切断面の上側か?
        if is_on_upside(y[v],x[v],ay,ax,by,bx) {
            // 上側とvを結合
            uft.unite(v, cut_up);
            // 下側とnvを結合
            uft.unite(nv, cut_down);
        }
        else {
            // 下側とvを結合
            uft.unite(v, cut_down);
            // 上側とnvを結合
            uft.unite(nv, cut_up);
        }
    }

    let mut set = BTreeSet::new();
    for i in 0..num_vertex {
        let r = uft.root(i);
        set.insert(r);
    }
    println!("{}", set.len());

}

fn is_on_upside(y: isize, x: isize, ay: isize, ax: isize, by: isize, bx: isize, ) -> bool {
    // 頂点(y,x)が、線分ABより上側にいるか判定

    if ay == by {
        return y >= ay
    }
    if ax == bx {
        // 右側をtrueとしておく
        return ax <= x
    }
    fn get_gradient(y0: isize, x0: isize, y1: isize, x1: isize) -> f64 {
        return (y0-y1) as f64 / (x0-x1) as f64
    }
    fn get_y_intercept(y0: isize, x0: isize, gradient: f64) -> f64 {
        return y0 as f64 - gradient * x0 as f64
    }
    let gradient = get_gradient(ay, ax, by, bx);
    let y_intercept = get_y_intercept(ay, ax, gradient);

    let y_on_line = gradient * x as f64 + y_intercept;
    return y as f64 >= y_on_line
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
}

fn get_cross_in_line_segment(y0: isize, x0: isize, y1: isize, x1: isize, y2: isize, x2: isize, y3: isize, x3: isize) -> Option<(f64, f64)> {
    // 線分p0-p1 と線分p2-p3 が交わるか判定 (直線ではないので注意)
    if let Some((y,x)) = get_cross(y0,x0,y1,x1,y2,x2,y3,x3) {
        fn is_between_p0_and_p1(x: f64, y: f64, y0: isize, x0: isize, y1: isize, x1: isize) -> bool {
            let y_min = y0.min(y1) as f64;
            let y_max = y0.max(y1) as f64;
            let x_min = x0.min(x1) as f64;
            let x_max = x0.max(x1) as f64;
            return y_min <= y && y <= y_max && x_min <= x && x <= x_max
        }
        if is_between_p0_and_p1(x, y, y0, x0, y1, x1) && is_between_p0_and_p1(x, y, y2,x2,y3,x3) {
            return Some((y,x))
        }
        else {
            return None
        }
    }
    else {
        return None
    }
}


fn get_cross(y0: isize, x0: isize, y1: isize, x1: isize, y2: isize, x2: isize, y3: isize, x3: isize) -> Option<(f64, f64)> {
    // 直線p0-p1 と直線p2-p3 が交わるか判定 (線分ではないので注意)
    if is_cross(y0,x0,y1,x1,y2,x2,y3,x3) {
        fn get_gradient(y0: isize, x0: isize, y1: isize, x1: isize) -> f64 {
            return (y0-y1) as f64 / (x0-x1) as f64
        }
        fn get_y_intercept(y0: isize, x0: isize, gradient: f64) -> f64 {
            return y0 as f64 - gradient * x0 as f64
        }

        if x0 == x1 && x2 == x3 {
            return Some((y0.min(y1) as f64, x0 as f64))
        }
        else if x0 == x1 {
            let a2 = get_gradient(y2,x2,y3,x3);
            let b2 = get_y_intercept(y2,x2,a2);

            let x = x0 as f64;
            let y = a2*x+b2;
            return Some((y,x))
        }
        else if x2 == x3 {
            let a0 = get_gradient(y0,x0,y1,x1);
            let b0 = get_y_intercept(y0,x0,a0);    

            let x = x2 as f64;
            let y = a0*x+b0;
            return Some((y,x))        
        }

        let a0 = get_gradient(y0,x0,y1,x1);
        let b0 = get_y_intercept(y0,x0,a0);

        let a2 = get_gradient(y2,x2,y3,x3);
        let b2 = get_y_intercept(y2,x2,a2);

        // a0*x+b0 = a2*x+b2
        let x = (b2-b0) / (a0-a2);
        let y = a0*x+b0;
        return Some((y,x))
    }
    else {
        return None
    }
}


fn is_cross(y0: isize, x0: isize, y1: isize, x1: isize, y2: isize, x2: isize, y3: isize, x3: isize) -> bool {
    // 直線p0-p1 と直線p2-p3 が交わるか判定 (線分ではないので注意)
    if x0 == x1 {
        return (x2 - x0) * (x3 - x0) <= 0
    }
    if y0 == y1 {
        return (y2 - y0) * (y3 - y0) <= 0
    }
    let f = |y: isize,x: isize| -> isize {
        return (y1-y0)*x - (x1-x0)*y - x0*y1 + x1*y0
    };
    if f(y2, x2) * f(y3, x3) < 0 {
        return true
    }
    else {
        let judge = |y: isize, x: isize| -> bool {
            return 
                min(y0,y1) <= y && y <= max(y0,y1) && 
                min(x0,x1) <= x && x <= max(x0,x1)
        };
        if f(y2, x2) == 0 {
            return judge(y2, x2)
        }
        else {
            return judge(y3, x3)
        }
    }
    // 線分p0-p1上の点を、(y,x)とする。
    // (x-x0):(y-y0) = (x-x1):(y-y1)
    // <=>  (y-y0) * (x-x1) = (x-x0)*(y-y1)
    // <=> (y1-y0)x - (x1-x0)y - x0*y1 + x1*y0 = 0
    // f(x,y) = (y1-y0)x - (x1-x0)y - x0*y1 + x1*y0 とおくと、
    // 線分p0-p1上の点(y,x)は、
    // f(x,y) = 0 
    // を満たす。

    // 線分p0-p1 と線分p2-p3 が交わるか判定するには、
    // f(x2,y2) * f(x3,y3) < 0
    // となっていればok
    // f(x2,y2) * f(x3,y3) = 0 となるときは厄介で、
    // f(x2,y2) = 0 または f(x3,y3) = 0 となっている。
    // f(x2,y2) = 0 のとき、min(y0,y1) <= y2 <= max(y0,y1) かつ min(x0,x1) <= x2 <= max(x0,x1) なら良い
    // f(x3,y3) = 0 のとき、min(y0,y1) <= y3 <= max(y0,y1) かつ min(x0,x1) <= x3 <= max(x0,x1) なら良い

}