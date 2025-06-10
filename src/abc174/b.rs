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
        d: isize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
    }

    let mut ans = 0;
    for i in 0..n {
        if x[i] * x[i] + y[i] * y[i] <= d * d {
            ans += 1;
        }
    }
    println!("{}", ans);
}