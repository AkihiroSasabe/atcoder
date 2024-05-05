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
        s: Chars,
        t: Chars
    }
    let mut x = 0;

    // let mut ans = vec![];
    for i in 0..t.len() {
        if t[i] == s[x] {
            x += 1;
            print!("{} ", i+1);
            if x == s.len() {break}
        }
    }



}