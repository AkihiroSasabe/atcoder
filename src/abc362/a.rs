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
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }
    let mut ans = 10000;

    if c == "Red" {
        ans = min(ans, g);
        ans = min(ans, b);
    } 
    if c == "Green" {
        ans = min(ans, r);
        ans = min(ans, b);
    }
    if c == "Blue" {
        ans = min(ans, r);
        ans = min(ans, g);
    }
    println!("{}", ans);

}