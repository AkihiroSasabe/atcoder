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
    s.reverse();
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut hash = HashMap::new();
    for i in 0..26 {
        hash.insert(uppercase[i], i+1);
    }

    let mut ans = 0;
    let mut exp = 1;
    for i in 0..s.len() {
        ans += hash[&s[i]] * exp;
        exp *= 26;
    }
    println!("{}", ans);

}