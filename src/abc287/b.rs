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
        m: usize,
        s: [usize; n],
        t: [usize; m]
    }
    let mut hash = HashMap::new();
    for i in 0..m {
        hash.insert(t[i], 0);
    }
    let mut ans = 0;
    for i in 0..n {
        let ss = s[i] % 1000;
        if hash.contains_key(&ss) {
            ans += 1;
        }
    }
    println!("{}", ans);
}