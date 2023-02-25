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
        n: usize
    }
    let mut count = vec![0; 2_000_000];
    for i in 1..n+1 {
        for j in 1..((n+1)/i + 1) {
            count[i*j] += 1;
        }
    }

    let mut ans = 0;    
    for i in 1..n+1 {
        ans += count[i] * count[n-i];
    }
    println!("{}", ans);

}