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
        q: usize,
        mut r: [usize; n],
        x: [usize; q]
    }
    r.sort();
    let mut cum = vec![0; n];
    
    cum[0] = r[0];
    for i in 1..n {
        cum[i] = cum[i-1] + r[i];
    }

    for i in 0..q {
        let ind = cum.upper_bound(&x[i]);
        println!("{}", ind);
    }
}