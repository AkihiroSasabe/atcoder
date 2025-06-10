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
        t: Chars, // ブブbん
    }

    let mut ans = t.len();
    for i in 0..s.len() {
        if i + t.len() - 1 > s.len() - 1 {break}

        let mut temp = 0;
        for j in 0..t.len() {
            if s[i+j] != t[j] {
                temp += 1;
            }           
        }
        ans = min(ans, temp);
    }
    println!("{}", ans);

}