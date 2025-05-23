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
        d: [(usize, usize); n]
    }

    for i in 0..n-2 {
        if d[i].0 == d[i].1 && d[i+1].0 == d[i+1].1 && d[i+2].0 == d[i+2].1 {
            println!("Yes");
            return;
        }
    }
            println!("No");

}