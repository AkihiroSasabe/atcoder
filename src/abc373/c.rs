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
use rand::Rng;
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut amax = a[0];
    let mut bmax = b[0];

    for i in 0..n {
        amax = max(amax, a[i]);
        bmax = max(bmax, b[i]);
    }
    println!("{}", amax + bmax);

}