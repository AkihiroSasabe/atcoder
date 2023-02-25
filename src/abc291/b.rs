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
        mut x: [usize; 5*n]
    }
    x.sort();

    let mut xx = vec![];
    for i in 0..x.len() {
        xx.push(x[i] as f64);
    }

    let mut ans = 0.0;
    for i in n..4*n {
        ans += xx[i];
    }
    ans = ans / ((3 * n) as f64);
    println!("{}", ans);

}