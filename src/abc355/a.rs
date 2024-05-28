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
        a: usize,
        b: usize,
    }

    // let mut hannin = vec![false; 3];

    if a == b {
        println!("-1");
        return;
    }
    for i in 1..4 {
        if i != a && i != b {
            println!("{}", i);
            return
        }
    }


}