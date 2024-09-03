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
use rand::Rng;
fn main() {
    input! {
        n: usize
    }
    let mut a = vec![];
    let mut s = vec![];
    for i in 0..n {
        input! {
            ai: isize,
            si: char,
        }
        a.push(ai);
        s.push(si);
    }

    let mut ans = 0;
    let init = 100000;
    let mut pos_l = init;
    let mut pos_r = init;

    for i in 0..n {
        if s[i] == 'L' {
            if pos_l != init {
                ans += (a[i] - pos_l).abs();
            }
            pos_l = a[i];
        }
        else {
            if pos_r != init {
                ans += (a[i] - pos_r).abs();
            }
            pos_r = a[i];
        }
    }
    println!("{}", ans);

}