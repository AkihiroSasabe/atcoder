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
    // 2024-01-03 16:24-16:30 (6min)
    // 2024-01-03 16:32-17:13 (41min)
    // total 47min
    input! {
        n: usize
    }

    let p7 = 10_000_001;
    let mut ans = 0;

    let mut hash = HashSet::new();
    for x in 1..min(p7, n+1) {
        // n / i == x となるものの個数
        let over = n / x;
        let under = 1 + n / (x+1);
        let num = if under <= over {
            1 + over - under
        }
        else {
            0
        };
        // println!("over = {over}, under = {under}");
        // println!("x = {x}, num = {num}");
        hash.insert(x);
        ans += num * x;
    }

    for i in 1..min(p7, n + 1) {
        let x = n / i;
        if hash.contains(&x) {
            break
        }
        ans += x;
    }
    println!("{}", ans);
    
    
}