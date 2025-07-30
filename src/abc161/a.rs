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
        // x: usize,
        // y: usize,
        // z: usize,
        mut a: [usize ;3]
    }
    a.swap(0,1);
    a.swap(0,2);
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

}