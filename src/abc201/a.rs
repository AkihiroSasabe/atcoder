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
    // 開始時刻: 2025-05-30 20:04:00
    input! {
        mut a: [usize; 3]
    }
    a.sort();
    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    }
    else {
        println!("No");

    }

}