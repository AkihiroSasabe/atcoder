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
        n: usize,
        s: Chars
    }
    let mut flag = false;
    let mut hash = HashMap::new();
    let mut x: isize = 0;
    let mut y: isize = 0;
    let digit = 200_010;
    hash.insert(x + y * digit, 0);
    for i in 0..s.len() {
        if s[i] == 'R' {
            x += 1;
        }
        else if s[i] == 'L' {
            x -= 1;
        }
        else if s[i] == 'U' {
            y += 1;
        }
        else if s[i] == 'D' {
            y -= 1;
        }
        if hash.contains_key(&(x + y * digit)) {
            flag = true;
            break
        }
        else {
            hash.insert(x + y * digit, 0);
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}