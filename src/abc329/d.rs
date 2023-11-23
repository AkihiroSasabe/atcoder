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
    // 2023-11-18
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }
    let mut heap = BinaryHeap::new();
    let mut hyo = vec![0; n];
    for i in 0..m {
        hyo[a[i] - 1] += 1;
        let v = vec![hyo[a[i] - 1], - ((a[i] - 1) as isize)];
        heap.push(v);
        let v2 = heap.pop().unwrap();
        println!("{}", - v2[1] + 1);
        heap.push(v2);
    }
}