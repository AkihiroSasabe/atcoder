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

    let sum = a.iter().sum::<usize>();

    let mut is_ok = false;
    for i in 0..n {
        if sum == a[i] + m {
            is_ok = true;
        }
    }
    if is_ok {
        println!("Yes");
    }
    else {
        println!("No");
    }

}