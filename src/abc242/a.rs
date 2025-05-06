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
    // 2025-05-03 11:53-
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    if x <= a {
        println!("1.0");
    }
    else if x > b {
        println!("0.0");
    }
    else {
        let bottom = b - (a + 1) + 1;
        let ans = c as f64 / bottom as f64;
        println!("{}", ans);
    }
}