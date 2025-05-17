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
        x: usize,
        ab: [(usize, usize); n]
    }
    let mut pos = vec![vec![false; 20000]; n+1];
    pos[0][0] = true;
    for i in 0..n {
        let ai = ab[i].0;
        let bi = ab[i].1;
        for p in 0..x+1 {
            pos[i+1][ai+p] |= pos[i][p];
            pos[i+1][bi+p] |= pos[i][p];
        }
    }
    if pos[n][x] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}