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
        a: [isize; n]
    }

    let mut cum = vec![0; n];
    cum[0] = a[0];
    let mut min_c = a[0];
    for i in 1..n {
        cum[i] = cum[i-1] + a[i];
        min_c = min(min_c, cum[i]);
    }
    if min_c < 0 {
        println!("{}", cum[n-1] - min_c);
    }
    else {
        println!("{}", cum[n-1]);
    }

}