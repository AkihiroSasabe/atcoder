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
        a: isize,
        m: isize,
        l: isize,
        r: isize,
    }

    let mut right = 0;
    let mut left = 0;
    let mut center = 0;
    let mut ans = 0;

    if l < a && a < r {
        center = 1;
        left = (a - l) / m;
        right = (r - a) / m;
        ans = right + center + left;
    }
    else if a < l {
        ans = (r - a) / m - (l - 1 - a) / m;
    }
    else if r < a {
        ans = (a - l) / m - (a - 1 - r) / m;
    }
    else if a == l {
        ans = 1 + (r - a) / m;
    }
    else if a == r {
        ans = 1 + (a - l) / m;
    }
    println!("{}", ans);

}