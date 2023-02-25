#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut hash = HashMap::new();
    for i in 0..n {
        hash.insert(a[i], 0);
    }
    let mut ans = k;
    for i in 0..k {
        if !hash.contains_key(&i) {
            ans = i;
            break
        }
    }
    println!("{}", ans);

}