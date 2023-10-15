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
        n: isize,
        mut a: [isize; n]
    }
    let mut sum = 0;
    for i in 0..n {
        sum += a[i as usize];
    }

    let mean = sum / n;
    let remain = sum % n;
    a.sort();
    a.reverse();
    let mut ans = 0;
    for i in 0..n {
        if i < remain {
            ans += ((mean + 1) - a[i as usize]).abs();
        }
        else {
            ans += (mean - a[i as usize]).abs();
        }
    }
    println!("{}", ans / 2);



}