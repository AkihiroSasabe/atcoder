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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        mut s: Chars
    }
    for i in 0..(s.len()/2) {
        let a = s[2*i];
        let b = s[2*i+1];
        s[2*i] = b;
        s[2*i+1] = a;
    }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }




}