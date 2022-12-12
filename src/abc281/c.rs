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
        t: usize,
        a: [usize; n],
    }
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
    }
    let mut left_time = t % sum;
    for i in 0..n {
        if left_time > a[i] {
            left_time -= a[i];
        }
        else {
            println!("{} {}", i+1, left_time);
            break
        }
    }
}