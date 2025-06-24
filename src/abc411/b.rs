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
        d: [usize; n-1],
    }

    let mut cum = d.clone();
    cum.insert(0, 0);
    for i in 1..n {
        cum[i] = cum[i] + cum[i-1];
    }
    // println!("cum = {:?}", cum);

    for i in 0..n {
        for j in i+1..n {
            print!("{} ", cum[j] - cum[i]);
        }
        if i == n-1 {continue;}
        println!("");
    }

}