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
    }

    let mut a = vec![];
    let mut c = vec![];
    for i in 0..n {
        input!{
            ai: usize,
            ci: usize,
        }
        a.push(ai);
        c.push(ci);
    }

    let mut btree = BTreeMap::new();

    for i in 0..n {
        btree.entry(c[i]).or_insert(BTreeSet::new()).insert(a[i]);
    }

    let mut ans = 0;
    for (ci, set) in btree {
        let min_v = *set.iter().next().unwrap();
        ans = max(ans, min_v);
    }
    println!("{}", ans);
}