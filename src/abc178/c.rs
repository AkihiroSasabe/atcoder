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
    }
    let modulus = 1_000_000_007;

    // 0 が存在しない
    // 9 が存在しない
    // 0 も 9 も存在しない
    // 両方存在するかもしれない。

    let mut s0 = 1;
    let mut s9 = 1;
    let mut s09 = 1;
    let mut s = 1;

    for i in 0..n {
        s *= 10;
        s %= modulus;
        s0 *= 9;
        s0 %= modulus;
        s09 *= 8;
        s09 %= modulus;
    }
    s9 = s0;

    let ans: isize = (s - s0 - s9 + s09 + modulus * 2) % modulus;
    println!("{}", ans);


}