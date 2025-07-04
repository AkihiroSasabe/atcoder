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
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut cum_a = a.clone();
    for i in 1..n {
        cum_a[i] = cum_a[i-1] + cum_a[i];
    }

    let mut cum_b = b.clone();
    for i in 1..m {
        cum_b[i] = cum_b[i-1] + cum_b[i];
    }
    cum_a.insert(0,0);
    cum_b.insert(0,0);
    let mut ans = 0;


    // a を何冊読むかで、全探索すればok
    for num_a in 0..n+1 {
        if k < cum_a[num_a] {break}
        let rem = k - cum_a[num_a];
        let ind = cum_b.upper_bound(&rem);
        let num_b = ind - 1;
        let cand = num_a + num_b;
        ans = max(ans, cand);
    }
    println!("{}", ans);
}