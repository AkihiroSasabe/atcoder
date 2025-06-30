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
        t: Chars,
    }

    let mut set = BTreeSet::new();
    for ti in t {
        set.insert(ti);
    }

    let mut is_ok = true;
    for i in 1..s.len() {
        if s[i].is_uppercase() {
            if !set.contains(&s[i-1]) {
                is_ok = false;
            }
        }
    }
    if is_ok {
        println!("Yes");
    }
    else {
        println!("No");

    }

}