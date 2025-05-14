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
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }
    let mut b = vec![vec![0; h]; w];

    for y in 0..h {
        for x in 0..w {
            b[x][y] = a[y][x]
        }
    }
    for y in 0..w {
        for x in 0..h {
            print!("{} ", b[y][x]);
        }
        println!("");
    }



}