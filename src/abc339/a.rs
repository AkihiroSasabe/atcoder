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
        s: Chars
    }

    let n = s.len();
    let mut last_dot = 0;
    for i in 0..n {
        if s[i] == '.' {
            last_dot = max(i, last_dot);
        }
    }
    for i in last_dot+1..n {
        print!("{}", s[i]);
    }

}