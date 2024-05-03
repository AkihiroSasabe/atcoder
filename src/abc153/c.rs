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
        k: usize,
        mut h: [usize; n]
    }
    let mut ans = 0;
    h.sort();
    h.reverse();
    // for i in 0..min(k, n) {
    //     ans += 1;
    // }
    for i in k..n {
        ans += h[i];
    }
    println!("{}", ans);
}