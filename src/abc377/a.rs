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
        s: Chars
    }
    let mut set = HashSet::new();
    for si in s {
        if si == 'A' || si == 'B' || si == 'C' {
            set.insert(si);
        }
    }
    if set.len() == 3 {
        println!("Yes");
    }
    else {
        println!("No");
    }

}