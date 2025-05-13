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
        m: usize,
        a: [usize; n],
    }
    let mut set = BTreeSet::new();

    let mut nums: Vec<usize> = vec![];
    for i in 0..n {
        set.insert(a[i]);
        nums.push(set.len());
    }

    let mut count = 0;
    for i in (0..n).rev() {
        if nums[i] != m {
            println!("{}", count);
            return;
        }
        count += 1;
    }
    println!("{}", count);



}