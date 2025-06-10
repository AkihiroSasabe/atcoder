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
    let modulus = 1_000_000_007;
    let mut cum = a.clone().iter().map(|x| x % modulus).collect::<Vec<usize>>();
    for i in 1..n {
        cum[i] = cum[i] + cum[i-1];
        cum[i] %= modulus;
    }

    let mut ans = 0;;
    for i in 0..n {
        ans += (modulus + cum[n-1] - cum[i]) * a[i] % modulus;
        ans %= modulus;
    }
    println!("{}", ans);

}