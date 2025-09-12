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

    let mut s0 = s[0] as usize - '0' as usize;
    let mut s1 = s[2] as usize - '0' as usize;
    if s1 < 8 {
        s1 += 1;
    }
    else {
        s1 = 1;
        s0 += 1;
    }

    println!("{}-{}", s0, s1);
}