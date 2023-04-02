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
        s: [Chars; 8]
    }
    let mut y = 0;
    let mut x = 0;
    for h in 0..8 {
        for w in 0..8 {
            if s[h][w] == '*' {
                y = 8 - h;
                x = w;
            }
        }
    }
    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    println!("{}{}",lowercase[x],  y);
}