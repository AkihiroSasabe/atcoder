#![allow(dead_code, unused_imports)]
// use proconio::{input, marker::{Usize1, Isize1}};
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
// use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    // 2025-02-07 19:43-20:14 (31min)
    let n = read_usize();

    // 100回聞いていい.
    // 2<=n<=50
    // 1<=wi<=10^6

    // 3回聞けば、一番長いエッジに含まれていない頂点が、
    // 中心位置にいることがわかる。
    // v0-v1, v1-v2, v2-v0

    let mut max_d = 0;
    let mut max_v = n;
    for i in 2..=n {
        let d = ask(1, i);
        if max_d <= d {
            max_v = i;  
            max_d = d;
        }
    }
    // println!("max_v = {:?}", max_v);
    // println!("max_d = {:?}", max_d);
    let mut ans = 0;
    for i in 1..=n {
        if i == max_v {continue}
        let d = ask(max_v, i);
        ans = max(d, ans);
    }

    println!("! {}", ans);


}

// インタラクティブな読み込みをする関数 (1行に1個のusize)
fn read_usize() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn ask(a: usize, b: usize) -> usize {
    println!("? {} {}", a, b);
    let dist = read_usize();
    return dist
}