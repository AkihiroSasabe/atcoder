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
    // 2024-04-20 21:43-22:40 (57min)
    // 2024-04-21 10:00-12:08 (2h8m)
    // Total 3h5m
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }

    
    // nを0にするのに、必要な最小のお金の期待値を求めたい。
    let mut dp = BTreeMap::new();
    let init_e = std::f64::MAX;

    // 候補の金額を全列挙して、setに格納。
    let mut set = BTreeSet::new();
    let devisors = vec![2,3,4,5,6];
    let mut queue = VecDeque::new();
    queue.push_back(n);
    while queue.len() != 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..devisors.len() {
            let di = devisors[i];
            let nv = v / di;
            if set.contains(&nv) {continue}
            queue.push_back(nv);
            set.insert(nv);
            dp.insert(nv, init_e);
        }
    }

    set.insert(n);
    dp.insert(n, init_e);   // 求めるNを0にするコストは、不明なので、maxで初期化
    dp.insert(0, 0.0);      // 0を0にするコストは、0

    // 2つの級数の公式を知っておく必要がある。
    // [1]等比級数
    // S = Σ[T=0,∞] r^T = 1 / (1 - r)
    // [2]名前不明の級数
    // S = Σ[T=0,∞] (T+1) * r^T = 1 / (1 - r)^2
    
    // 数値Nの期待値
    // (1/6)^0 * (1/6*(E6+Y) + 1/6*(E5+Y) + 1/6*(E4+Y) + 1/6*(E3+Y) + 1/6*(E2+Y))
    // + (1/6)^1 * (1/6*(E6+2*Y) + 1/6*(E5+2*Y) + 1/6*(E4+2*Y) + 1/6*(E3+2*Y) + 1/6*(E2+2*Y))
    // + (1/6)^2 * (1/6*(E6+3*Y) + 1/6*(E5+3*Y) + 1/6*(E4+3*Y) + 1/6*(E3+3*Y) + 1/6*(E2+3*Y))
    // + ....

    for &v in set.iter() {
        *dp.get_mut(&v).unwrap() = 
            dp.get(&v).unwrap()
            .min(dp.get(&(v / a)).unwrap() + x as f64);

        let mut sum = 0.0;
        for i in 0..devisors.len() {
            let di = devisors[i];
            let nv = v / di;
            sum += *dp.get(&nv).unwrap() / 6.0
        }
        sum += y as f64;
        let e_cost = sum * 6.0 / 5.0;
        *dp.get_mut(&v).unwrap() = 
            dp.get(&v).unwrap()
            .min(e_cost);
    }
    println!("{}", dp.get(&n).unwrap());
}
