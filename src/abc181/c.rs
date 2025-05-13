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
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
    }
    // nC3
    for comb in (0..n).combinations(3) {
        let x0 = x[comb[0]];
        let y0 = y[comb[0]];

        let x1 = x[comb[1]];
        let y1 = y[comb[1]];

        let x2 = x[comb[2]];
        let y2 = y[comb[2]];

        // 同一直線上?

        let dx1 = x1 - x0;
        let dy1 = y1 - y0;

        let dx2 = x2 - x0;
        let dy2 = y2 - y0;

        if dx1 * dy2 == dx2 * dy1 {
            println!("Yes");
            return;
        }
    }
    println!("No");

}