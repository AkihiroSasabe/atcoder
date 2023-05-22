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
        a: [usize; n]
    }
    let mut ans = vec![];
    ans.push(a[0]);
    for i in 1..n {
        if a[i-1] < a[i] {
            for j in a[i-1]+1..a[i]+1 {
                ans.push(j);
            }
        }
        else {
            for k in 1..(1+a[i-1]-a[i]) {
                ans.push(a[i-1] - k);
            }
        }
    }
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}