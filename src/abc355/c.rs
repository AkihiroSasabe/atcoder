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
        n: usize,
        t: usize,
        a: [usize; t],
    }

    let mut xs = vec![0; n];
    let mut ys = vec![0; n];
    let mut aa = 0; // \
    let mut bs = 0; // /

    for i in 0..t {
        let x = (a[i] - 1) % n;
        let y = (a[i] - 1) / n;
        xs[x] += 1;
        ys[y] += 1;

        if x == y {
            aa += 1;
        }
        if y == n - 1 - x {
            bs += 1;
        }

        if xs[x] == n || ys[y] == n || aa == n || bs == n {
            println!("{}", i+1);
            return
        }
    }
    println!("-1");

}