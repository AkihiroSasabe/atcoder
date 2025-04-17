#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive};
use num_bigint::ToBigUint;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let modulus = 1_000_000_000;
    let mut a = vec![0; n+1];
    let mut cum = vec![0; n+1];
    a[0] = 1;
    for i in 1..min(k, n+1) {
        a[i] = 1;
        cum[i] = a[i] + cum[i-1];
        cum[i] %= modulus;
    }
    if k < n + 1 {
        a[k] = k;
        cum[k] = cum[k-1] + a[k];
        cum[k] %= modulus;
    }
    for i in k+1..n+1 {
        a[i] = modulus + cum[i-1] - cum[i-k-1];
        a[i] %= modulus;
        
        cum[i] = cum[i-1] + a[i];
        cum[i] %= modulus;
    }
    println!("{}", a[n]);

}