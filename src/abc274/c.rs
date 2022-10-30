#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut order = vec![0; 2 * n + 2];
    for i in 0..n {
        order[2 * (i+1)] = order[a[i]] + 1;
        order[2 * (i+1) + 1] = order[a[i]] + 1
    }

    for i in 1..(2*n + 2) {
        println!("{}", order[i]);
    }

}

// fn dfs() {

// }