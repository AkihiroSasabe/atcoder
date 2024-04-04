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
    // 2024-04-03 21:14-21:46 (32min)
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }


    // xiが与えられる
    let mut a = vec![0; n];
    
    // S = ∅
    // i個目のクエリ
    // - Sからxiを削除 
    // - Sに　xiを追加
    // if j in S => a[j] += |S| (j = 0,1,...,n-1)


    let mut btree = BTreeMap::new();
    let mut cum = vec![0; q];
    let mut a = vec![0; n];
    for i in 0..q {
        let xi = x[i] - 1;
        if btree.contains_key(&xi) {
            // 削除
            let in_time = btree.remove(&xi).unwrap();
            if in_time != 0 {
                a[xi] += cum[i-1] - cum[in_time-1];
            }
            else {
                a[xi] += cum[i-1];
            }
            cum[i] = cum[i-1] + btree.len();
        }
        else {
            // 追加
            btree.insert(xi, i);
            if i == 0 {
                cum[i] = btree.len();
            }
            else {
                cum[i] = cum[i-1] + btree.len();
            }
        }
    }
    for i in 0..n {
        if btree.contains_key(&i) {
            let in_time = btree.remove(&i).unwrap();
            if in_time == 0 {
                a[i] += cum[q-1];
            }
            else {
                a[i] += cum[q-1] - cum[in_time - 1];
            }
        }
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }

}