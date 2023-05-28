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
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    }
    let mut naka = vec![vec![false; n]; n];
    for i in 0..m {
        for j in 0..n-1 {
            naka[a[i][j] - 1][a[i][j+1] - 1] = true;
            naka[a[i][j+1] - 1][a[i][j] - 1] = true;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            if !naka[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

}