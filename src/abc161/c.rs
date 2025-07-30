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
        mut n: isize,
        k: isize
    }

    let mut ans = n;
    if n > k {
        n = n - (n / k) * k;
        ans = min(ans, n)
    }

    if ans == 0 {
        println!("0");
        return;
    }

    for _ in 0..10 {
        n = (k - n).abs();
        ans = min(ans, n);
    }
    println!("{}", ans);

    




}