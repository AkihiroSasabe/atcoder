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
        t: usize,
        c: [usize; n],
        r: [usize; n]
    }
    let mut in_t = false;

    let mut cand = vec![];
    let mut cand2 = vec![];
    for i in 0..n {
        if c[i] == t {
            in_t = true;
            cand.push(vec![r[i], i+1])
        }
        if c[i] == c[0] {
            cand2.push(vec![r[i], i+1])
        }
    }
    cand.sort();
    cand.reverse();
    cand2.sort();
    cand2.reverse();

    if in_t {
        println!("{}", cand[0][1]);
    }
    else {
        println!("{}", cand2[0][1]);
    }


}