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
    let mut min_index = 0;
    let mut min_a = 1_000_000_000_000;
    let mut s = vec![];
    let mut a = vec![];
    for i in 0..n {
        input! {
            s_i: String,
            a_i: usize
        }
        s.push(s_i);
        // a.push(vec![a_i, i]);
        a.push(a_i);
        if min_a >= a_i {
            min_a = a_i;
            min_index = i;
        }
    }
    for i in 0..n {
        let index = (min_index + i) % n;
        println!("{}", s[index]);
    }
    
}