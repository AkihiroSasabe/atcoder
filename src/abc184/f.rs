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
    // 2024-05-01 14:58-15:22 (24min, 詰み)
    // 15:31-15:49 (18min, 解説見た。)
    // 半分全列挙 
    input! {
        n: usize,
        t: isize,
        mut a: [isize; n],
    }
    // T分以下で、最大になるようにする。
    // DPは使えそうにない。
    // 1_099_511_627_776 = 10^12 なのでbit全探索も無理
    // にぶたんでは?

    // ok = min(A)
    // ng = a.iter().sum();

    // ソートと累積和
    a.sort();

    // dpっぽくいけない?
    // 137_846_528_820 = 10^11

    // let mut aaa = [2_usize; 2];

    // 答え見た。半分全列挙
    // let mut a0 = &a[0..n/2];
    // let mut a1 = &a[n/2..];

    let mut sum0 = BTreeSet::new();
    let mut sum1 = BTreeSet::new();

    let hn: usize = n / 2;
    let mut a0 = vec![];
    let mut a1 = vec![];
    for i in 0..n {
        if i < hn {
            a0.push(a[i]);
        }
        else {
            a1.push(a[i]);
        }
    }

    for mask in 0..(1 << a0.len()) {
        let mut sum = 0;
        for i in 0..a0.len() {
            if mask & (1 << i) != 0 {
                sum += a0[i];
            }
        }
        sum0.insert(sum);
    }
    for mask in 0..(1 << a1.len()) {
        let mut sum = 0;
        for i in 0..a1.len() {
            if mask & (1 << i) != 0 {
                sum += a1[i];
            }
        }
        sum1.insert(sum);
    }

    let mut ans = 0;
    for sum in sum0 {
        let tg = t - sum;
        if tg < 0 {
            continue
        }
        if let Some(cand) = sum1.range(..tg+1).rev().next() {
            // println!("sum={sum}, cand={cand}");
            ans = max(ans, sum + cand);
        }
    }
    println!("{}", ans);
}