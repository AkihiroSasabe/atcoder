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
    // 2025-05-17 12:08- ばちゃ

    input! {
        a: usize,
        b: usize,
    }

    if min(a,b) + 1 == max(a,b) || (min(a,b) == 1 && max(a,b) == 10) {
        println!("Yes");
    }
    else {
        println!("No");

    }

}