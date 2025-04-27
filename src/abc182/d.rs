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
        a: [isize; n],
    }
    let mut ans = 0;

    let mut cum = a.clone();
    for i in 1..n {
        cum[i] = cum[i] + cum[i-1];
    }

    let mut pos = a[0];
    let mut ans = max(0, a[0]);
    let mut max_diff = max(0, a[0]);
    for i in 1..n {
        max_diff = max(max_diff, cum[i]);
        ans = max(ans, pos + max_diff);
        pos = pos + cum[i];
    }

    println!("{}", ans);
}