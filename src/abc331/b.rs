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
        s: usize,
        m: usize,
        l: usize
    }

    let mut ans = (n / 12 + 2) * l;
    for i in 0..(n/6 + 2) {
        for j in 0..(n/8 + 2) {
            for k in 0..(n/12 + 2) {
                if i * 6 + j * 8 + k * 12 >= n {
                    let p = i * s + j * m + k * l;
                    ans = min(ans, p);
                }
            }
        }
    }
    println!("{}", ans);

}