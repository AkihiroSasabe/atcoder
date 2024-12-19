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
fn main() {
    // 2024-12-13 19:50-20:23 (33min)
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }
    // 要素の積がk以下

    let mut is_all_over_k = true;
    for i in 0..n {
        if s[i] <= k {
            is_all_over_k = false;
        }
        if s[i] == 0 {
            // 0 が一個でもあれば、Nにできる。
            println!("{}", n);
            return;
        }
    }
    if is_all_over_k {
        println!("0");
        return;
    }

    // 累積積は使っちゃいけなさそう。
    // 尺取かな

    let mut ans = 1;
    let mut deque = VecDeque::new();
    let mut p = 1;
    let mut min_r = 0;
    for l in 0..n {
        // println!("l = {:?} -------", l);
        loop {
            if let Some(&v) = deque.front() {
                if v < l {
                    let v = deque.pop_front().unwrap();
                    p /= s[v];
                }
                else if v == l {break}
            }
            else {
                // 無ければ追加
                deque.push_back(l);
                p *= s[l];
                break
            }
        }
        if p > k {continue}
        min_r = max(l, min_r);

        // println!("deque = {:?}", deque);
        // println!("min_r = {:?}", min_r);
        // println!("p = {:?}", p);

        // 先頭が l な状況は作った。
        for r in min_r+1..n {
            min_r = r;
            p *= s[r];
            deque.push_back(r);
            if p > k {
                p /= s[r];
                // println!("p = {:?}", p);
                min_r = r-1;
                deque.pop_back();
                break
            }
        }
        ans = max(ans, deque.len());
    }
    println!("{}", ans);
}