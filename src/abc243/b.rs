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
        b: [usize; n],
    }
    let mut ans  = 0;
    let mut ans2  = 0;

    for i in 0..n {
        if a[i] == b[i] {
            ans += 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {continue}
            if a[i] == b[j] {
                ans2 += 1;
            }    
        }
    }
    println!("{}", ans);
    println!("{}", ans2);

}