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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = vec![];
    for i in 0..n {
        c.push((a[i], 0));
    }
    for j in 0..m {
        c.push((b[j], 1));
    }
    c.sort();

    for i in 1..n+m {
        if c[i].1 == 0 && c[i-1].1 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");


}