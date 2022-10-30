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
        h: [usize; n]
    }

    let mut ans = 0;
    let mut max_ind = 0;
    for i in 0..n {
        if ans < h[i] {
            ans = h[i];
            max_ind = i;
        }
    }
    println!("{}", max_ind + 1);
}