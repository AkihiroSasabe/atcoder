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
        y: usize
    }

    let mut ans = 365;
    if y % 4 != 0 {
        ans = 365;
    }
    else if y % 4 == 0 && y % 100 != 0 {
        ans = 366;
    }
    else if y % 100 == 0 && y % 400 != 0 {
        ans = 365;
    }
    else if y % 400 == 0 {
        ans = 366;
    }
    println!("{}", ans);


}