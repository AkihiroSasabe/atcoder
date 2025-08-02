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
    // ばちゃ
    // 2025-08-02 10:44-
    input! {
        a: [usize; 3]
    }
    let sum = a.iter().sum::<usize>();
    if sum >= 22 {
        println!("bust");
    } else {
        println!("win");
    }
}