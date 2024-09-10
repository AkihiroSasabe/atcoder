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
use rand::Rng;
fn main() {
    // 2024-09-10 19:06-20:08 (1h2min, 解説みた)
    // 2024-09-10 20:08-20:21 (13min)
    // Total: 1h15min
    input! {
        n: usize,
        mut a: [isize; n],
    }

    // 2 <= N <= 50
    // |a[i]| <= 10^6

    // a[y] += a[x]
    // a[0] <= ... <= a[n-1]

    // 0 以上, 2N回
    // 一番右に、一番デカいやつを足していけばいいのでは?

    // -2, 5, -1
    // -2, 5, 4
    // -2, 5, 9

    // 5, 3, -1000
    // -995, 3, -1000
    // -1995, 3, -1000
    // -1995, -1992, -1000

    // 一番左に一番小さいやつを足すか、一番右に一番デカいやつを足すか?

    let max_a = *a.iter().max().unwrap();
    let min_a = *a.iter().min().unwrap();

    let mut max_ind = 0;
    let mut min_ind = 0;
    for i in 0..n {
        if a[i] == max_a {
            max_ind = i;
        }
        if a[i] == min_a {
            min_ind = i;
        }
    }

    let mut ans = vec![];
    if min_a.abs() <= max_a.abs() {
        // 全て正にする
        for i in 0..n {
            if i == max_ind {continue}
            a[i] += max_a;
            ans.push((max_ind + 1, i + 1));
        }
        ans.push((max_ind + 1, max_ind + 1));
        for i in 1..n {
            ans.push((i-1 +1, i + 1));
        }
    }
    else {
        // 全て負にする
        for i in 0..n {
            if i == min_ind {continue}
            a[i] += min_a;
            ans.push((min_ind + 1, i + 1));
        }
        ans.push((max_ind + 1, max_ind + 1));
        for i in (1..n).rev() {
            ans.push((i + 1, i-1 + 1));
        }
    }
    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }



}