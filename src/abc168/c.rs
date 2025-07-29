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
        a: isize,
        b: isize,
        h: isize,
        m: isize,
    }

    // ((h + m / 60) / 12 - m / 60) * 360 = 30*h + m/2 - 6*m = 30h-11/2m
    let ang_deg_x2 = (60*h-11*m);
    let ang_deg_x2 = if ang_deg_x2 >= 360 {
        720 - ang_deg_x2
    } else {
        ang_deg_x2
    };
    let ans = ((a*a + b*b) as f64 - (2*a*b) as f64 *(ang_deg_x2 as f64 * PI / 360.0).cos()).sqrt();
    println!("{}", ans);
}