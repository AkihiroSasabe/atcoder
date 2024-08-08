#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        xa: isize,
        ya: isize,
        xb: isize,
        yb: isize,
        xc: isize,
        yc: isize,
    }

    let e_ab = get_len(xa, ya, xb, yb);
    let e_bc = get_len(xc, yc, xb, yb);
    let e_ca = get_len(xa, ya, xc, yc);

    let mut es = vec![e_ab, e_bc, e_ca];
    es.sort();

    if es[2] == es[0] + es[1] {
        println!("Yes");
    }
    else {
        println!("No");

    }


}

fn get_len(x0: isize, y0: isize, x1: isize, y1: isize) -> isize {
    (x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1)
}