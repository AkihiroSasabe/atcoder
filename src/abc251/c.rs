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
    }
    let mut s = vec![];
    let mut t = vec![];
    for i in 0..n {
        input! {
            s_i: String,
            t_i: usize
        }
        s.push(s_i);
        t.push(t_i);
    }

    // オリジナル
    // 点数がmax
    // 提出が早い

    let mut max_t = 0;
    let mut ans = 0;
    let mut hash = HashMap::new();
    for i in 0..n {
        if !hash.contains_key(&s[i]) {
            hash.insert(&s[i], 1);
            if t[i] > max_t {
                max_t = t[i];
                ans = i + 1;
            }
        }
    }
    // println!("{:?}", hash);
    println!("{}", ans);


}