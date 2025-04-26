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
        s: [Chars; n]
    }
    let mut btree = BTreeMap::new();
    for si in s {
        *btree.entry(si).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (name, num) in btree {
        heap.push((num, name));
    }
    if let Some((num, name)) = heap.pop() {
        for nn in name {
            print!("{}", nn);
        }
    }

}