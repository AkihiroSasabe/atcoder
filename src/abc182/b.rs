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

    let mut ans = 0;
    let mut gcd_level = 0;
    for k in 2..1001 {
        let mut cand = 0;
        for i in 0..n {
            if a[i] % k == 0 {
                cand += 1;
            }
        }
        if gcd_level <= cand {
            gcd_level = cand;
            ans = k;
        }
    }
    println!("{}", ans);
}