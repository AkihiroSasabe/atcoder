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
    let mut n = s.len();

    let mut hash = HashSet::new();
    for length in 1..n+1 {
        for start in 0..n {
            let end = start + length;
            if end > n {continue}
            let mut temp = vec![];
            for i in start..end {
                temp.push(s[i]);
            }
            if temp.len() == 0 {continue}
            hash.insert(temp);
        }
    }
    println!("{}", hash.len());

}