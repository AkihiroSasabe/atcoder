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
    let mut a = vec![];
    let mut c = vec![];
    for i in 0..n {
        input!{
            c_i: usize,
            a_i: [usize; c_i],
        }
        c.push(c_i);
        a.push(a_i);
    }
    input! {
        x: usize,
    }

    let mut btree = BTreeMap::new();
    for i in 0..n {
        if a[i].contains(&x) {
            btree.entry(c[i]).or_insert(vec![]).push(i);
        }
    }
    if btree.len() == 0 {
        println!("0");
    }
    else {
        let (mut min_c, mut min_b) = btree.iter_mut().next().unwrap();
        min_b.sort();
        println!("{}", min_b.len());
        for i in 0..min_b.len() {
            print!("{} ", min_b[i] + 1);
        }
    }

}