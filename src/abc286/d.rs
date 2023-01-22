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
        x: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }
        a.push(a_i);
        b.push(b_i);
    }
    // let max_money = 100*50*50+1;
    // let INF = 100_000_000;
    let INF = x+1;
    let mut dp = vec![false; INF];
    dp[0] = true;
    // println!("{:?}", dp);
    for i in 0..n {
        let dp_temp = dp.clone();
        for j in 0..x+1 {
            if dp_temp[j] {
                for k in 0..b[i]+1 {
                    if j+k*a[i] > x {break}
                    // println!("j+k*a[i]: {}", j+k*a[i]);
                    dp[j+k*a[i]] = true;
                }
            }
        }
        // println!("{:?}", dp);
    }
    if dp[x] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}