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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut a = vec![];
    for i in 0..m {
        input! {
            c_i: usize,
            a_i: [usize; c_i]
        }
        a.push(a_i);
    }
    let mut ans = 0;
    for bit in 0..(1_usize << m) {
        let mut hash = HashMap::new();
        for i in 0..m {
            // ビット演算
            if bit & (1 << i) != 0 {
                for j in 0..a[i].len() {
                    let v = a[i][j];
                    if !hash.contains_key(&v) {
                        hash.insert(v, 1);
                    }
                }
            }
        }
        if hash.len() == n {
            ans += 1;
        }
    }
    println!("{}", ans);
}