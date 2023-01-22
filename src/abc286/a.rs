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
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [usize; n]
    }
    let mut b = a.clone();
    for (i, val) in (p-1..q).enumerate() {
        // println!("{} {}", i, val);
        b[r-1+i] = a[val];
        b[val] = a[r-1+i];
    }
    for i in 0..n {
        print!("{} ", b[i]);
    }


}