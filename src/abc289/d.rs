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
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize
    }
    let mut moti = HashMap::new();
    for i in 0..b.len() {
        moti.insert(b[i], 1);
    }

    let mut dp = vec![false; x+1];
    dp[0] = true;
    for dan in 0..x+1 {
        for i in 0..n {
            if dp[dan] {
                if a[i]+dan <= x && !moti.contains_key(&(a[i]+dan)) && !moti.contains_key(&dan) {
                    dp[a[i]+dan] = true;
                }
            }
        }
    }
    if dp[x] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}