#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut ans = t.len();
    for i in 0..s.len() {
        if s[i] != t[i] {
            ans = i + 1;
            break
        }
    }
    println!("{}", ans);

}