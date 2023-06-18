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
        a: [usize; 3*n]
    }
    let mut ans = vec![vec![]; n];
    for i in 0..3*n {
        ans[a[i]-1].push(i+1);
    }
    // println!("{:?}", ans);
    let mut f = vec![];
    for i in 0..n {
        f.push(vec![ans[i][1], i+1]);
    }
    f.sort();
    for i in 0..n {
        print!("{} ", f[i][1]);
    }



}