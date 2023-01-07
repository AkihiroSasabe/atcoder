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
        t: usize,
    }
    let mut n = vec![];
    let mut a = vec![];
    for i in 0..t {
        input! {
            n_i: usize,
            a_i: [usize; n_i]
        }
        n.push(n_i);
        a.push(a_i);
    }
    for i in 0..a.len() {
        let mut count = 0;
        for j in 0..a[i].len() {
            if a[i][j] % 2 == 1 {
                count += 1;
            }
        }
        println!("{}", count);
    }
}