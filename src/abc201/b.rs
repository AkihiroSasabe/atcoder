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
    // let mut s = vec![];
    // let mut t = vec![];
    let mut ts = vec![];
    for i in 0..n {
        input!{
            si: Chars,
            ti: usize,
        }
        // s.push(si);
        // t.push(ti);
        ts.push((ti, si));
    }
    ts.sort();
    ts.reverse();
    println!("{}", ts[1].1.iter().collect::<String>());

}