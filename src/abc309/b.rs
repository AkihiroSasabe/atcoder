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
        a: [Chars; n]
    }
    let mut b = a.clone();
    b[0][0] = a[1][0];
    b[0][n-1] = a[0][n-2];
    b[n-1][0] = a[n-1][1];
    b[n-1][n-1] = a[n-2][n-1];


    for i in 0..n-1 {
        b[0][i+1] = a[0][i];
    }
    for i in 0..n-1 {
        b[i+1][n-1] = a[i][n-1];
    }
    for i in 1..n {
        b[n-1][i-1] = a[n-1][i];
    }
    for i in 1..n {
        b[i-1][0] = a[i][0];
    }
    for i in 0..b.len() {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        if i == n - 1 {continue}
        println!("");
    }
    

}