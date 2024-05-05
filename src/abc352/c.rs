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
    }
    let mut max_a = 0;
    let mut sum_b = 0;
    for i in 0..n {
        input!{
            ai: usize,
            bi: usize,
        }
        max_a = max(max_a, bi - ai);
        sum_b += ai;
    }
    println!("{}", max_a + sum_b);

}