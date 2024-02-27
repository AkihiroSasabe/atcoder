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
        s: Chars
    }
    let mut hash = HashMap::new();
    for i in 0..s.len() {
        *hash.entry(s[i]).or_insert(0) += 1;
        // s[i]
    }
    let mut c = 'a';
    for (xx, num) in hash.iter() {
        if *num == 1 {
            c = *xx;
        }
    }

    for i in 0..s.len() {
        if s[i] == c {
            println!("{}", i+1);
        }
    }
}