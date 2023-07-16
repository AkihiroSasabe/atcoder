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
        k: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut ab = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
        ab.push(vec![a_i, b_i]);
    }

    ab.sort();
    let mut sum = 0;
    for i in 0..n {
        sum += b[i];
    }

    let mut day = 1;
    for i in 0..n+1 {
        // println!("i={}, sum={}, day={}", i, sum, day);
        if sum <= k {
            println!("{}", day);
            return
        }
        sum -= ab[i][1];
        day = ab[i][0] + 1;
    }


}