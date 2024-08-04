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
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut sum = 0;
    let mut cum = vec![0; n];
    cum[0] = a[0];
    for i in 0..n {
        sum += a[i];
        if i > 0 {
            cum[i] = cum[i-1] + a[i];
        }
    }

    if sum <= m {
        println!("infinite");
        return
    }
    // println!("a = {:?}", a);
    // println!("cum = {:?}", cum);

    // めぐる式二分探索
    let mut ng = a[n-1] + 1;
    let mut ok = 0;
    while (ng as i128 - ok as i128).abs() > 1 {
        let mid = (ng + ok) / 2;
        let is_ok = judge(mid, &a, &cum, m, n);
        // println!("mid = {:?}", mid);
        // println!("is_ok = {:?}", is_ok);
        if is_ok {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn judge(x: usize, a: &Vec<usize>, cum: &Vec<usize>, m: usize, n: usize) -> bool {
    // xより大きいやつ
    let ind = a.upper_bound(&x);
    if ind == n {
        return cum[n-1] <= m
    }
    if ind == 0 {
        return x * n <= m
    }
    return cum[ind-1] + x * (n - ind) <= m
}