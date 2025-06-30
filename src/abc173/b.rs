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
        n: usize,
        s: [String; n]
    }

    let a = ["AC", "WA", "TLE", "RE"];

    let mut c = vec![0; 4];
    for i in 0..n {
        for j in 0..4 {
            if s[i] == a[j] {
                c[j] += 1;
            }
        }
    }
    for i in 0..4 {
        println!("{} x {}", a[i], c[i]);
    }



}