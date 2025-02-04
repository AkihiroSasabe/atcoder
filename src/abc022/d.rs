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
    // 2025-02-02 19:52-20:39 (47min)
    // 2025-02-03 19:43-21:05 (1h22min, 途中で答え見た。)
    // Total: 2h9min
    input! {
        n: usize
    }
    let mut ax = vec![];
    let mut ay = vec![];
    for i in 0..n {
        input!{
            axi: f64,
            ayi: f64,
        }
        ax.push(axi);
        ay.push(ayi);
    }

    let mut bx = vec![];
    let mut by = vec![];
    for i in 0..n {
        input!{
            bxi: f64,
            byi: f64,
        }
        bx.push(bxi);
        by.push(byi);
    }

    // 自分のオリジナルの解法。重心を中心に考えて、半径基準で最もスパースな点をAとBからそれぞれ取得し、それらが対応することからPを求めた。
    // 重心からの距離rが異なるが、スパース度が等しい点があるので、WAになっていると思われる。
    // 重心を使う発想は悪くなかった。
    // solve(n, &mut ax, &mut ay, &mut bx, &mut by);

    // 重心からの最大距離の比が、P。 (答え見てしまった。)
    solve2(n, &mut ax, &mut ay, &mut bx, &mut by); 
}

fn solve2(n: usize, ax: &mut Vec<f64>, ay: &mut Vec<f64>, bx: &mut Vec<f64>, by: &mut Vec<f64>) {
    // let mut agx: f64 = ax.iter().sum::<f64>() / (n as f64);
    let agx = get_mass_of_center(&ax);
    let agy = get_mass_of_center(&ay);

    let bgx = get_mass_of_center(&bx);
    let bgy = get_mass_of_center(&by);

    // println!("agx = {:?}", agx);
    // println!("agy = {:?}", agy);

    // println!("bgx = {:?}", bgx);
    // println!("bgy = {:?}", bgy);

    // 全てを重心が原点に来るように移動
    for i in 0..n {
        ax[i] -= agx;
        ay[i] -= agy;

        bx[i] -= bgx;
        by[i] -= bgy;
    }

    // 原点からの距離を格納
    let mut ar  = vec![];
    let mut br = vec![];
    for i in 0..n {
        ar.push(((ax[i] * ax[i] + ay[i] * ay[i]).sqrt(), i));
        br.push(((bx[i] * bx[i] + by[i] * by[i]).sqrt(), i));
        // br.push(bx[i] * bx[i] + by[i] * by[i]);
    }

    // もっとも疎な位置にいるarを知りたい
    // ar.sort();
    // br.sort();

    ar.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    br.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());

    ar.reverse();
    br.reverse();

    let max_dist_a = ar[0].0;
    let max_dist_b = br[0].0;

    let ans = max_dist_b / max_dist_a;
    println!("{}", ans);


}


fn solve(n: usize, ax: &mut Vec<f64>, ay: &mut Vec<f64>, bx: &mut Vec<f64>, by: &mut Vec<f64>) {
    // let mut agx: f64 = ax.iter().sum::<f64>() / (n as f64);
    let agx = get_mass_of_center(&ax);
    let agy = get_mass_of_center(&ay);

    let bgx = get_mass_of_center(&bx);
    let bgy = get_mass_of_center(&by);

    // println!("agx = {:?}", agx);
    // println!("agy = {:?}", agy);

    // println!("bgx = {:?}", bgx);
    // println!("bgy = {:?}", bgy);

    // 全てを重心が原点に来るように移動
    for i in 0..n {
        ax[i] -= agx;
        ay[i] -= agy;

        bx[i] -= bgx;
        by[i] -= bgy;
    }

    // 原点からの距離を格納
    let mut ar  = vec![];
    let mut br = vec![];
    for i in 0..n {
        ar.push(((ax[i] * ax[i] + ay[i] * ay[i]).sqrt(), i));
        br.push(((bx[i] * bx[i] + by[i] * by[i]).sqrt(), i));
        // br.push(bx[i] * bx[i] + by[i] * by[i]);
    }

    // もっとも疎な位置にいるarを知りたい
    // ar.sort();
    // br.sort();

    ar.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    br.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());

    // 最もスパースな点の取得
    let ind_a = get_most_sparse_index(&ar);
    let ind_b = get_most_sparse_index(&br);

    // ind_a と ind_b が対応していることがわかる。
    
    // こいつを起点に、頑張る
    let axi = ax[ind_a];
    let ayi = ay[ind_a];

    let bxi = bx[ind_b];
    let byi = by[ind_b];

    let ans = ((bxi * bxi + byi * byi) / (axi * axi + ayi * ayi)).sqrt();
    println!("{}", ans);

}


// 最もスパースな点の取得
fn get_most_sparse_index(ar: &Vec<(f64, usize)>) -> usize {
    let mut max_dist = 0.0;
    let mut max_dist_ind = 0;
    let n = ar.len();

    

    // 前後の距離のうち、小さい方が、最大のものを求める。
    for i in 0..n {
        // 前後の距離
        let mut pre = 100000000000000.0;
        let mut next = 100000000000000.0;
        if i != 0 {
            // 前の距離
            pre = ar[i].0 - ar[i-1].0;
        }
        if i != n - 1 {
            // 後ろの距離
            next = ar[i+1].0 - ar[i].0;
        }
        let mut dist = pre;
        if next < pre {
            dist = next;
        }
        if max_dist < dist {
            max_dist_ind = ar[i].1;
            max_dist = dist;
        }
    }
    return max_dist_ind

}


// 重心の取得
fn get_mass_of_center(ax: &Vec<f64>) -> f64 {
    let n = ax.len();
    let mut agx: f64 = ax.iter().sum::<f64>() / (n as f64);
    return agx
}