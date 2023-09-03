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
        abcd: [[usize; 4]; n]
    }
    let mut s = vec![vec![0; 200]; 200];

    let mut ans = 0;
    for i in 0..n {
        let x_min = abcd[i][0];
        let x_max = abcd[i][1];
        let y_min = abcd[i][2];
        let y_max = abcd[i][3];
        for x in x_min..x_max {
            for y in y_min..y_max {
                s[y][x] = 1;
            }
        }
    }

    for y in 0..200 {
        for x in 0..200 {
            ans += s[y][x];
        }
    }

    println!("{}", ans);

    
}