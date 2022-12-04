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
        s: [isize; n]
    }
    for i in 0..n {
        if i == 0 {
            print!("{} ", s[0]);
        }
        else {
            print!("{} ", s[i]-s[i-1]);
        }
    }
}