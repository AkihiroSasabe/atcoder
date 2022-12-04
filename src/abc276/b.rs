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
        m: usize,
    }
    // let mut a = vec![];
    // let mut b = vec![];
    let mut toshi = vec![vec![]; n];
    for i in 0..m {
        input! {
            a_i: usize,
            b_i: usize,
        }
        toshi[a_i - 1].push(b_i - 1);
        toshi[b_i - 1].push(a_i - 1);
        // a.push(a_i);
        // b.push(b_i);
    }
    for i in 0..n {
        let length = toshi[i].len();
        print!("{} ", length);
        toshi[i].sort();
        for j in 0..length {
            print!("{} ", toshi[i][j] + 1);
        }
        println!("");
    }
}