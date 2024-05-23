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
        h: usize
    }

    let mut hh = 0;
    let mut ans = 0;
    let mut diff = 1;
    loop {
        if h < hh {
            println!("{}", ans);
            return;
        }
        ans += 1;
        hh += diff;
        diff = diff * 2;
    }

}