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

    for i in 0..1_000 {
        let v0 = rand::thread_rng().gen_range(0..n);
        let v1 = rand::thread_rng().gen_range(0..n);
        // println!("(v0, v1) = {:?}", (v0, v1));
        if v0 == v1 {continue}

        let x0 = x[v0];
        let y0 = y[v0];
        let x1 = x[v1];
        let y1 = y[v1];

        let dx = x1 - x0;
        let dy = y1 - y0;

        let mut count = 0;
        for j in 0..n {
            let tx = x[j] - x0;
            let ty = y[j] - y0;
            if dx * ty == dy * tx {
                count += 1;   
            }
        }
        if count >= (n + 1) / 2 {
            println!("Yes");

            // 直交するので。
            // (y-y0)*dx + (x-x0)*dy == 0
            // <=> dy*x + dx*y + ( -dy*x0 - dx*y0) = 0

            // x = dx*t + x0; ---(1)
            // y = dy*t + y0; ---(2)

            // (1) * dy - (2) * dx
            // (x-x0)*dy - (y-y0)*dx = 0
            // dy*x - dx*y + (dx*y0 - dy*x0) = 0

            let (a,b,c) = (dy,-dx, dx*y0 - dy*x0);
            println!("{} {} {}", a, b, c);
            return;
        }
    }
    println!("No");
}