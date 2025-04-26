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
    // 2025-04-25 0:29-
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }
    let d0 = v * t;
    let d1 = v * s;

    if d0 <= d && d <= d1 {
        println!("No");
    }
    else {
        println!("Yes");

    }


}