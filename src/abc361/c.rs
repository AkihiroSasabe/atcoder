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
        mut a: [usize; n],
    }

    // 貪欲?
    a.sort();

    // simulation
    let num = n - k;

    let mut ans = 1_000_000_001;
    for i in 0..k+1 {
        let temp = a[num-1+i] - a[i];
        ans = min(ans, temp);
    }
    println!("{}", ans);

}