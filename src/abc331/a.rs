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
        M: usize,
        D: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize,
    }

    let mut d2 = (d + 1) % D;
    if d2 == 0 {
        d2 = D;
    }
    let mut y2 = y;
    let mut m2 = m;
    if d + 1 > D {
        m2 = (m + 1) % M;
        if m2 == 0 {
            m2 = M;
        }
        if m + 1 > M {
            y2 += 1;
        }
    }
    println!("{} {} {}", y2, m2, d2);

}