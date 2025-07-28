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
        s: Chars,
    }
    let n = s.len();
    let mut  t = s.clone();

    let mut is_ok = true;
    for i in 0..n {
        if s[i] == '#' {
            is_ok = true;
            continue
        }
        if is_ok {
            t[i] = 'o';
            is_ok = false;
        }
    }
    println!("{}", t.iter().collect::<String>());
}