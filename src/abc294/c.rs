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
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = vec![];
    for i in 0..n {
        c.push(a[i]);
    }
    for i in 0..m {
        c.push(b[i]);
    }
    c.sort();
    let mut hash = HashMap::new();
    for i in 0..n+m {
        hash.insert(c[i], i+1);
    }
    for i in 0..n {
        print!("{} ", hash[&a[i]]);
    }
    println!("");
    for i in 0..m {
        print!("{} ", hash[&b[i]]);
    }
}