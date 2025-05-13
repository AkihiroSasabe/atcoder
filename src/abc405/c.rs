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
        a: [usize; n],
    }
    let mut cum = a.clone();
    for i in 1..n {
        cum[i] = cum[i-1] + cum[i];
    }

    let mut ans = 0;
    for i in 0..n-1 {
        ans += a[i] * (cum[n-1] - cum[i]);
    }
    println!("{}", ans);

}