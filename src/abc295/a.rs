#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        n: usize,
        w: [String; n]
    }
    let mut dict = vec!["and", "not", "that", "the", "you"];
    // let mut hash = HashMap::new();
    // hash.insert(k, v)
    let mut flag = false;
    for i in 0..n {
        for j in 0..dict.len() {
            if w[i] == dict[j] {
                flag = true;
            }
        }
    }
    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}