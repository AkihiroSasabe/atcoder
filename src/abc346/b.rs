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
        w: usize,
        b: usize,
    }
    // 12文字
    let mut s: Vec<char> = "wbwbwwbwbwbw".chars().collect();

    // println!("{}", s.len());
    let mut ss= vec![];
    for _ in 0..25 {
        for i in 0..s.len() {
            ss.push(s[i]);
        }
    }

    for l in 0..ss.len()-(w+b) {
        let mut num_w = 0;
        let mut num_b = 0;
        for i in l..l+w+b {
            if ss[i] == 'w' {
                num_w += 1;
            }
            else {
                num_b += 1;
            }
        }
        if num_w == w && num_b == b {
            println!("Yes");
            return
        }
    }
    println!("No");




}