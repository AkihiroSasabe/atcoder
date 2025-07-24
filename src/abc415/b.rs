#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    input! {
        s: Chars
    }
    let mut anss =vec![];
    for i in 0..s.len() {
        if s[i] == '#' {
            anss.push(i+1);
        } 
    }
    for i in 0..anss.len() {
        if i % 2 == 1 {
            println!("{},{}", anss[i-1], anss[i]);
        }
    }

}