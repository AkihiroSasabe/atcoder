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
use rand::Rng;
fn main() {
    input! {
        s: [Chars; 8]
    }
    let mut y_set = vec![];
    for y in 0..8 {
        let mut is_ok = true;
        for x in 0..8 {
            if s[y][x] == '#' {
                is_ok = false;
            } 
        }
        if is_ok {
            y_set.push(y);
        }
    }

    let mut x_set = vec![];
    for x in 0..8 {
        let mut is_ok = true;
        for y in 0..8 {
            if s[y][x] == '#' {
                is_ok = false;
            } 
        }
        if is_ok {
            x_set.push(x);
        }
    }
    let ans = y_set.len() * x_set.len();
    println!("{}", ans);

}