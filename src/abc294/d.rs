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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        q: usize,

    }
    let mut yobare = BTreeMap::new();
    let mut yobaretenai = vec![];
    for i in 0..n {
        yobaretenai.push(n-i);
    }
    for i in 0..q {
        input! {
            kind: usize
        }
        if kind == 1 {
            let v = yobaretenai.pop().unwrap();
            yobare.insert(v, v);
        }
        if kind == 2 {
            input! {
                x_i: usize
            }
            yobare.remove(&x_i);
        }
        if kind == 3 {
            let (k, v) = yobare.range(..).next().unwrap();
            println!("{}", k);
        }
    }
}